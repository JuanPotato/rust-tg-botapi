#!/usr/bin/env python3

import textwrap
import pathlib
import json
import re

type_map = {
    'Integer or String': 'ChatId',
    'InputFile or String': 'InputFile',
    'Boolean': 'bool',
    'True': 'bool',
    'Integer': 'i64',
    'Array of Integer': 'Vec<i64>',
    'Float': 'f64',
    'Float number': 'f64',
    'InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or ForceReply': 'ReplyMarkup',
    'InputMediaAudio, InputMediaDocument, InputMediaPhoto and InputMediaVideo': 'InputMedia',
    'Array of InputMediaAudio, InputMediaDocument, InputMediaPhoto and InputMediaVideo': 'Vec<InputMedia>',
    'Message or True': 'MessageOrBool',
    'Message or Boolean': 'MessageOrBool',
    'Message': 'Box<Message>',
}

PRIMITIVE = ['i64', 'f64', 'String', 'bool', 'ParseMode', 'ChatId', 'InputFile']


def to_camel_case(s):
    return ''.join(w.title() for w in re.findall(r'^[a-z0-9]+|[A-Z]+[a-z0-9]*', s))

def has_file(field, objects):
    o = objects.get(field.type.base_type)
    return field.type.base_type == 'InputFile' or (o and o.has_inputfile)

def desc_to_doc(desc, indent=0):
    spaces = ' ' * indent
    lines = textwrap.wrap(desc, width=100 - (indent + 5))
    return ''.join(f'{spaces}/// {l}\n' for l in lines)

class TgType:
    def __init__(self, type_str, name='', required=True):
        self.raw = type_str
        self.rust_type = type_map.get(self.raw, self.raw)

        if name == 'parse_mode':
            self.rust_type = 'ParseMode'

        while 'Array of' in self.rust_type:
            self.rust_type = re.sub('Array of (.+)', r'Vec<\1>', self.rust_type)

        self.inner_optional = self.rust_type
        if not required:
           self.rust_type = f'Option<{self.rust_type}>'
        
        self.base_type = self.rust_type
        while '<' in self.base_type:
            self.base_type = re.sub('\w+<(.+)>', r'\1', self.base_type)
    
    def __repr__(self) -> str:
        return self.rust_type

class TgField:
    def __init__(self, d):
        self.raw_name = d['field']
        self.type = TgType(d['type'], d['field'], d['required'])
        self.required = d['required']
        self.desc = d['description']
    
        self.name = self.raw_name
        if self.raw_name in ('type', 'Self', 'self'):
            self.name += '_'
        
        self.required_value = None
        if m := re.search(r", must be (\w+)$", self.desc):
            self.required_value = m.group(1)
        self.skip = False

    def __repr__(self) -> str:
        return f'{self.name}: {self.type.rust_type}'

class TgObject:
    def __init__(self, d):
        self.name = d['name']
        self.camel = to_camel_case(self.name)
        self.fields = [TgField(f) for f in d['fields']]
        self.desc = d['description']
        self.need_serialize = False
        self.has_inputfile = any(has_file(f, {}) for f in self.fields)
        self.is_enum_variant = False
    
    def get_tag(self):
        for f in self.fields:
            if f.required_value:
                return f
        return None
    
    def dump_def(self):
        defs = ''

        if self.need_serialize:
            defs += '#[derive(Debug, Clone, Serialize, Deserialize)]\n'
        else:
            defs += '#[derive(Debug, Clone, Deserialize)]\n'
        defs += desc_to_doc(self.desc)
        defs += f'pub struct {self.camel} {{\n'
        for field in self.fields:
            if field.skip: continue
            if field.name != field.raw_name:
                defs += f'    #[serde(rename = "{field.raw_name}")]\n'
            if not field.required and self.need_serialize:
                defs += f'    #[serde(skip_serializing_if = "Option::is_none")]\n'
            defs += desc_to_doc(field.desc, 4)
            defs += f'    pub {field.name}: {field.type.rust_type},\n'
        defs += f'}}\n\n'

        return defs
    
    def dump_impl(self, objects_map):
        impls = ''

        if self.need_serialize:
            if sum(f.required for f in self.fields) > 0:
                impls += self.dump_new()

            if sum(not f.required for f in self.fields) > 0:
                impls += self.dump_modifiers()
        
        if self.has_inputfile:
            impls += self.dump_add_file(objects_map)
        
        return impls
    
    def dump_new(self):
        output  = f'impl {self.camel} {{\n'
        output += f'    pub fn new('
        for field in self.fields:
            if field.skip: continue
            if field.required:
                output += f'{field.name}: {field.type.rust_type}, '
        output += f') -> Self {{\n'

        output += f'        Self {{\n'

        for field in self.fields:
            if field.skip: continue
            if field.required:
                output += f'            {field.name},\n'
            else:
                output += f'            {field.name}: None,\n'

        output += f'        }}\n'
        output += f'    }}\n'
        output += f'}}\n\n'

        return output
    
    def dump_modifiers(self):
        output  = f'impl {self.camel} {{\n'

        for field in self.fields:
            if field.skip: continue
            if not field.required:
                output += f'    pub fn with_{field.raw_name}(mut self, {field.name}: {field.type.inner_optional}) -> Self {{\n'
                output += f'        self.{field.name} = Some({field.name});\n'
                output += f'        self\n'
                output += f'    }}\n\n'

        output += f'}}\n\n'

        return output

    def dump_add_file(self, objects_map):
        output = ''
        output += f'impl crate::TgObject for {self.camel} {{\n'
        output += f'    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {{\n'
        for field in self.fields:
            if field.skip: continue
            if has_file(field, objects_map):
                output += f'        form = self.{field.name}.add_file(form);\n'
        output += f'        form\n'
        output += f"    }}\n"
        output += f"}}\n\n"
        return output

    def __repr__(self) -> str:
        return f'TgObject({self.name}, {self.fields})'

class TgMethod(TgObject):
    def __init__(self, d):
        super().__init__(d)
        self.ret = TgType(d['return'])
        self.need_serialize = True
    
    def dump_impl(self, objects_map):
        impls = ''
        impls += f'impl TgMethod for {self.camel} {{\n'
        impls += f'    type ResponseType = {self.ret.rust_type};\n'
        impls += f"    const PATH: &'static str = \"{self.name}\";\n\n"

        impls += f'    fn to_form(&self) -> reqwest::multipart::Form {{\n'
        impls += f'        let mut form = reqwest::multipart::Form::new();\n'
        for field in self.fields:
            if has_file(field, objects_map):
                impls += f'        form = self.{field.name}.add_file(form);\n'

        for field in self.fields:
            var = f"self.{field.name}"
            if not field.required:
                impls += f'        if let Some(s) = &self.{field.name} {{\n    '
                var = "s"
            if field.type.inner_optional in PRIMITIVE:
                impls += f'        form = form.text("{field.raw_name}", {var}.to_string());\n'
            else:
                impls += f'        form = form.text("{field.raw_name}", serde_json::to_string(&{var}).unwrap());\n'
            if not field.required:
                impls += f'        }}\n'
        

        impls += f"        form\n"
        impls += f"    }}\n"
        impls += f"}}\n\n"
        impls += super().dump_impl(objects_map)

        return impls

    def __repr__(self) -> str:
        return f'TgMethod({self.name}, {self.fields}) -> {self.ret}'

class TgEnum:
    def __init__(self, d):
        self.name = d['name']
        self.variants = d['variants']
        self.filtered = [filter_variant(v, self.name) for v in self.variants]
        self.need_serialize = False
        self.has_inputfile = False

    def dump_def(self, objects_map):
        tags = [tag for v in self.variants if (tag := objects_map[v].get_tag())]
        if tags:
            if not all(t.name == tags[0].name for t in tags) or len(tags) != len(self.variants):
                raise Exception("Somethings wrong...")

        defs = ''

        if self.need_serialize:
            defs += '#[derive(Debug, Clone, Serialize, Deserialize)]\n'
        else:
            defs += '#[derive(Debug, Clone, Deserialize)]\n'

        if tags:
            defs += f'#[serde(tag = "{tags[0].raw_name}")]\n'
        else:
            defs += '#[serde(untagged)]\n'

        defs += f'pub enum {self.name} {{\n'
        for i, (var, fil) in enumerate(zip(self.variants, self.filtered)):
            if tags:
                defs += f'    #[serde(rename = "{tags[i].required_value}")]\n'
                tags[i].skip = True
            defs += f'    {fil}({var}),\n'

        defs += f'}}\n\n'

        return defs

    def dump_impl(self, objects_map):
        impls = ''
        if self.has_inputfile:
            impls += self.dump_add_file(objects_map)

        for var, fil in zip(self.variants, self.filtered):
            impls += f'impl From<{var}> for {self.name} {{\n'
            impls += f'    fn from(o: {var}) -> Self {{\n'
            impls += f'        Self::{fil}(o)\n'
            impls += f'    }}\n'
            impls += f'}}\n\n'
    
        return impls

    def dump_add_file(self, objects_map):
        output = ''
        output += f'impl crate::TgObject for {self.name} {{\n'
        output += f'    fn add_file(&self, mut form: reqwest::multipart::Form) -> reqwest::multipart::Form {{\n'
        output += f'        match self {{\n'
        for var, fil in zip(self.variants, self.filtered):
            if objects_map[var].has_inputfile:
                output += f'            Self::{fil}(e) => e.add_file(form),\n'

        output += f'            _ => form,\n'
        output += f'        }}\n'
        output += f"    }}\n"
        output += f"}}\n\n"
        return output

    def __repr__(self) -> str:
        return f'TgEnum({self.name}, {self.variants})'
    
def main():
    output_path = pathlib.Path("./")

    with open('./api.json', 'r') as f:
        j = json.load(f)

    j['enums'].append({
        "name": "ReplyMarkup",
        "variants": [
            "InlineKeyboardMarkup",
            "ReplyKeyboardMarkup",
            "ReplyKeyboardRemove",
            "ForceReply"
        ]
    })

    enums = [TgEnum(o) for o in j['enums']]
    objects = [TgObject(o) for o in j['objects'] if o['name'] != 'Update']
    methods = [TgMethod(o) for o in j['functions']]

    enum_map = {e.name:e for e in enums}
    objects_map = {o.name:o for o in objects}
    e_o_map = {**enum_map, **objects_map}

    needed_serialize = set()
    need_serialize = list(methods)
    while need_serialize:
        m = need_serialize.pop()
        
        if m.name in needed_serialize:
            continue
        needed_serialize.add(m.name)

        if isinstance(m, TgObject):
            for f in m.fields:
                t = f.type.base_type
                
                if t in objects_map:
                    if t not in needed_serialize:
                        need_serialize.append(objects_map[t])
                    objects_map[t].need_serialize = True

                elif t in enum_map:
                    enum_map[t].need_serialize = True
                    for o in enum_map[t].variants:
                        if o not in needed_serialize:
                            need_serialize.append(objects_map[o])
                        objects_map[o].need_serialize = True

    has_inputfile = set()
    changed = True
    while changed:
        changed = False
        for o in objects:
            if o.has_inputfile:
                continue

            for f in o.fields:
                field_type = f.type.base_type
                if field_type in has_inputfile:
                    has_inputfile.add(o.name)
                    o.has_inputfile = True
                    changed = True
        
        for e in enums:
            if e.has_inputfile:
                continue

            for v in e.variants:
                if objects_map[v].has_inputfile:
                    has_inputfile.add(e.name)
                    e.has_inputfile = True
                    changed = True
    
    for m in methods:
        for f in m.fields:
            field_type = f.type.base_type
            fo = e_o_map.get(field_type)

            if fo and fo.has_inputfile:
                m.has_inputfile = True

    type_file = open(output_path / 'types.rs', 'w')
    type_file.truncate(0)
    type_impl_file = open(output_path / 'types_impl.rs', 'w')
    type_impl_file.truncate(0)

    meth_file = open(output_path / 'methods.rs', 'w')
    meth_file.truncate(0)
    meth_impl_file = open(output_path / 'methods_impl.rs', 'w')
    meth_impl_file.truncate(0)

    type_file.write('use crate::helpers::*;\n\n')

    type_impl_file.write('use crate::types::*;\n')
    type_impl_file.write('use crate::helpers::*;\n\n')

    meth_file.write('use crate::types::*;\n')
    meth_file.write('use crate::helpers::*;\n\n')

    meth_impl_file.write('use crate::methods::*;\n')
    meth_impl_file.write('use crate::types::*;\n')
    meth_impl_file.write('use crate::helpers::*;\n')
    meth_impl_file.write('use crate::{TgMethod, TgObject};\n\n')

    type_file.write(''.join(e.dump_def(e_o_map) for e in enums))
    type_file.write(''.join(o.dump_def() for o in objects))

    type_impl_file.write(''.join(e.dump_impl(e_o_map) for e in enums))
    type_impl_file.write(''.join(o.dump_impl(e_o_map) for o in objects))

    meth_file.write(''.join(m.dump_def() for m in methods))
    meth_impl_file.write(''.join(m.dump_impl(e_o_map) for m in methods))

def filter_variant(variant, type_name):
    # Nice job past me, this works really well
    WORD_RE = re.compile(r"[A-Z]+[a-z0-9]*")

    var_matches = WORD_RE.findall(variant)
    type_matches = WORD_RE.findall(type_name)

    v_index = 0
    t_index = 0

    v_len = len(var_matches)
    t_len = len(type_matches)

    unique = ''

    if t_index + 1 < t_len and var_matches[v_index] == type_matches[t_index + 1]:
        t_index += 1

    while True:
        if var_matches[v_index] == type_matches[t_index]:
            if t_index + 1 < t_len:
                t_index += 1
            else:
                while v_index + 1 < v_len:
                    v_index += 1
                    unique += var_matches[v_index]

            if v_index + 1 < v_len:
                v_index += 1
            else:
                break
            
        else:
            unique += var_matches[v_index]
            if v_index + 1 < v_len:
                v_index += 1
            else:
                break

    res = unique if unique else type_matches[t_len - 1]

    return res

if __name__ == '__main__':
    main()
