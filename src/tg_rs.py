#!/usr/bin/env python3

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

def main():
    output_path = pathlib.Path("./")

    with open('api.json', 'r') as f:
        j = json.load(f)

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
    type_impl_file.write('use crate::form_ser::*;\n')
    type_impl_file.write('use crate::helpers::*;\n\n')

    meth_file.write('use crate::types::*;\n')
    meth_file.write('use crate::helpers::*;\n\n')

    meth_impl_file.write('use crate::methods::*;\n')
    meth_impl_file.write('use crate::types::*;\n')
    meth_impl_file.write('use crate::form_ser::*;\n')
    meth_impl_file.write('use crate::helpers::*;\n')
    meth_impl_file.write('use crate::TgMethod;\n\n')

    defs, impls = dump_enums(j)
    type_file.write(defs)
    type_impl_file.write(impls)

    defs, impls = dump_types(j)
    type_file.write(defs)
    type_impl_file.write(impls)

    defs, impls = dump_methods(j)
    meth_file.write(defs)
    meth_impl_file.write(impls)

def dump_enums(j):
    defs = ''
    impls = ''

    for enum in j['enums']:
        defs +=  '#[derive(Debug, Clone, Deserialize)]\n'
        defs +=  '#[serde(untagged)]\n'
        defs += f'pub enum {enum["name"]} {{\n'
        for var in enum['variants']:
            defs += f'    {filter_variant(var, enum["name"])}({var}),\n'

        defs += f'}}\n\n'

        impls += f'impl FormSer for {enum["name"]} {{\n'
        impls += f'    fn serialize(&self, key: String, mut form: Form) -> Form {{\n'
        impls += f'        match self {{\n'

        for var in enum['variants']:
            impls += f'            {enum["name"]}::{filter_variant(var, enum["name"])}(e) => e.serialize(key, form),\n'

        impls += f"        }}\n"
        impls += f"    }}\n"
        impls += f"}}\n\n"

        for var in enum['variants']:
            var_name = filter_variant(var, enum["name"])
            impls += f'impl From<{var}> for {enum["name"]} {{\n'
            impls += f'    fn from(o: {var}) -> Self {{\n'
            impls += f'        Self::{var_name}(o)\n'
            impls += f'    }}\n'
            impls += f'}}\n\n'

    return defs, impls


def dump_types(j):
    defs = ''
    impls = ''

    for t in j['objects']:
        if t['name'] == 'Update':
            continue
        defs += write_struct(t)
        impls += write_form_ser(t)
        if sum(f['required'] for f in t['fields']) > 0:
            impls += write_new(t)

        if sum(not f['required'] for f in t['fields']) > 0:
            impls += write_modifiers(t)

    return defs, impls


def dump_methods(j):
    defs = ''
    impls = ''

    for m in j['functions']:
        name = camel(m['name'])

        defs += write_struct(m)

        impls += f'impl FormSer for {name} {{\n'
        impls += f'    fn serialize(&self, key: String, mut form: Form) -> Form {{\n'

        for field in m['fields']:
            f = fix_name(field['field'])
            impls += f'        form = self.{f}.serialize("{field["field"]}".into(), form);\n'

        impls += f"        form\n"
        impls += f"    }}\n"
        impls += f"}}\n\n"


        impls += f'impl TgMethod for {name} {{\n'
        impls += f'    type ResponseType = {norm_type(field["field"], m["return"])};\n'
        impls += f"    const PATH: &'static str = \"{m['name']}\";\n"
        impls += f"}}\n\n"

        if sum(f['required'] for f in m['fields']) > 0:
            impls += write_new(m)

        if sum(not f['required'] for f in m['fields']) > 0:
            impls += write_modifiers(m)


    return defs, impls

def write_struct(s):
    name = camel(s['name'])
    output  =  '#[derive(Debug, Clone, Deserialize)]\n'
    output += f'pub struct {name} {{\n'
    for field in s['fields']:
        f = fix_name(field['field'])
        t = norm_type(f, field['type'], not field['required'])
        if f != field['field']:
            output += f'\n    #[serde(rename = "{field["field"]}")]\n'
        output += f'    pub {f}: {t},\n'
    output += f'}}\n\n'

    return output

def write_form_ser(s):
    name = camel(s['name'])
    output  = f'impl FormSer for {name} {{\n'
    output += f'    fn serialize(&self, key: String, mut form: Form) -> Form {{\n'

    for field in s['fields']:
        f = fix_name(field['field'])
        output += f'        form = self.{f}.serialize(format!("{{}}[{field["field"]}]", key), form);\n'

    output += f"        form\n"
    output += f"    }}\n"
    output += f"}}\n\n"

    return output

def write_new(s):
    name = camel(s['name'])
    output  = f'impl {name} {{\n'
    output += f'    pub fn new('
    for field in s['fields']:
        f = fix_name(field['field'])
        t = norm_type(f, field['type'], not field['required'])
        if field['required']:
            output += f'{f}: {t}, '
    output += f') -> Self {{\n'

    output += f'        Self {{\n'

    for field in s['fields']:
        f = fix_name(field['field'])
        if field['required']:
            output += f'            {f},\n'
        else:
            output += f'            {f}: None,\n'

    output += f'        }}\n'
    output += f'    }}\n'
    output += f'}}\n\n'

    return output


def write_modifiers(s):
    name = camel(s['name'])
    output  = f'impl {name} {{\n'

    for field in s['fields']:
        f = fix_name(field['field'])
        t = norm_type(f, field['type'], not field['required'])

        if not field['required']:
            raw_type = norm_type(f, field['type'], False)
            output += f'    pub fn with_{f}(mut self, {f}: {raw_type}) -> Self {{\n'
            output += f'        self.{f} = Some({f});\n'
            output += f'        self\n'
            output += f'    }}\n\n'

    output += f'}}\n\n'

    return output


def fix_name(s):
    if s in ('type', 'Self', 'self'):
        return s + '_'
    return s

def norm_type(f, t, add_option=False):
    if f == 'parse_mode' and t == 'String':
        t = 'ParseMode'

    t = type_map.get(t, t)

    while 'Array of' in t:
        t = re.sub('Array of (.+)', r'Vec<\1>', t)

    if add_option:
        t = f'Option<{t}>'

    return t

def camel(s):
    return ''.join(w.title() for w in re.findall(r'^[a-z0-9]+|[A-Z]+[a-z0-9]*', s))

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
