use std::ffi::{c_char, CStr, CString};
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;

use redscript::bundle::{ConstantPool, ScriptBundle};
use redscript::definition::{AnyDefinition, Definition};
use redscript_decompiler::display::{display_definition_with_line_count, OutputMode};

#[no_mangle]
pub unsafe extern "C" fn bundle_load(path: *const c_char) -> Option<Box<ScriptBundle>> {
    let str = unsafe { CStr::from_ptr(path) }.to_string_lossy();
    let mut reader = BufReader::new(File::open(str.as_ref()).ok()?);
    Some(Box::new(ScriptBundle::load(&mut reader).ok()?))
}

#[no_mangle]
pub extern "C" fn bundle_free(_: Box<ScriptBundle>) {}

#[no_mangle]
pub unsafe extern "C" fn decompile_global(bundle: &ScriptBundle, name: *const c_char) -> Option<Box<Decompilation>> {
    let name = unsafe { CStr::from_ptr(name) }.to_string_lossy();

    let name_idx = bundle.pool.names.get_index(&name)?;
    let (_, def) = bundle
        .pool
        .roots()
        .find(|(_, def)| def.name == name_idx && matches!(def.value, AnyDefinition::Function(_)))?;

    Some(Decompilation::create(def, &bundle.pool)?.into())
}

#[no_mangle]
pub unsafe extern "C" fn decompile_method(
    bundle: &ScriptBundle,
    parent: *const c_char,
    name: *const c_char,
) -> Option<Box<Decompilation>> {
    let parent = unsafe { CStr::from_ptr(parent) }.to_string_lossy();
    let name = unsafe { CStr::from_ptr(name) }.to_string_lossy();

    let name_idx = bundle.pool.names.get_index(&name)?;
    let parent_idx = bundle.pool.names.get_index(&parent)?;
    let class = bundle
        .pool
        .roots()
        .find_map(|(_, def)| def.value.as_class().filter(|_| def.name == parent_idx))?;
    let method = class
        .methods
        .iter()
        .find_map(|&idx| bundle.pool.definition(idx).ok().filter(|def| def.name == name_idx))?;

    Some(Decompilation::create(method, &bundle.pool)?.into())
}

#[no_mangle]
pub extern "C" fn decompilation_code(decompilation: &Decompilation) -> *const c_char {
    decompilation.output.as_ptr()
}

#[no_mangle]
pub extern "C" fn decompilation_line_mapping(decompilation: &Decompilation) -> *const u32 {
    decompilation.lines.as_ptr()
}

#[no_mangle]
pub extern "C" fn decompilation_line_count(decompilation: &Decompilation) -> u32 {
    decompilation.lines.len() as u32
}

#[no_mangle]
pub extern "C" fn decompilation_free(_: Box<Decompilation>) {}

pub struct Decompilation {
    output: CString,
    lines: Vec<u32>,
}

impl Decompilation {
    fn create(def: &Definition, pool: &ConstantPool) -> Option<Self> {
        let (disp, line_count) = display_definition_with_line_count(def, pool, OutputMode::Code { verbose: false });
        let mut output = String::new();
        let mut line_counter = line_count.wrap_fmt(&mut output);
        write!(line_counter, "{}\0", disp).ok()?;

        let result = Decompilation {
            lines: line_counter.into_line_indices(),
            output: CString::from_vec_with_nul(output.into_bytes()).ok()?,
        };
        Some(result)
    }
}
