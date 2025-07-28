// #![windows_subsystem = "windows"] // uncomment to suppress console flash
use std::{env, ffi::OsString, os::windows::ffi::OsStrExt, ptr};
use serde::Serialize;
use windows::{
  core::*, Win32::Foundation::HWND, Win32::System::Com::*,
  Win32::UI::Shell::*, Win32::UI::Shell::Common::COMDLG_FILTERSPEC,
};

#[derive(Serialize)] struct Out { canceled: bool, paths: Vec<String> }

fn parse_arg(k:&str, args:&[String], def:&str)->String{
  args.windows(2).find(|w| w[0]==k).map(|w| w[1].clone()).unwrap_or_else(|| def.to_string())
}
fn has_flag(k:&str, args:&[String])->bool{ args.iter().any(|a| a==k) }

fn w(s:&str)->Vec<u16>{ OsString::from(s).encode_wide().chain([0]).collect() }

fn main() -> Result<()> { unsafe {
  CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE).ok();
  let args: Vec<String> = env::args().collect();
  let mode = parse_arg("--mode",&args,"files");          // files|folder
  let multi = has_flag("--multi",&args);
  let title = parse_arg("--title",&args,"Open");
  let initial = parse_arg("--initial",&args,"");
  let filter = parse_arg("--filter",&args,"");            // e.g. "*.srt;*.vtt;*.csv"

  let dlg: IFileOpenDialog = CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER)?;
  dlg.SetTitle(PCWSTR(w(&title).as_ptr()))?;
  let mut opts = dlg.GetOptions()?;
  opts |= FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST;
  if mode=="folder" { opts |= FOS_PICKFOLDERS; }
  if multi { opts |= FOS_ALLOWMULTISELECT; }
  dlg.SetOptions(opts)?;

  if !initial.is_empty() {
    if let Ok(item) = SHCreateItemFromParsingName::<_, _, IShellItem>(PCWSTR(w(&initial).as_ptr()), None) {
      dlg.SetFolder(&item).ok();
      dlg.SetDefaultFolder(&item).ok();
    }
  }

  if !filter.is_empty() {
    let spec = COMDLG_FILTERSPEC {
      pszName: PCWSTR(w("Files").as_ptr()),
      pszSpec: PCWSTR(w(&filter).as_ptr()),
    };
    dlg.SetFileTypes(&[spec])?;
  }

  let hr = dlg.Show(HWND(ptr::null_mut()));
  if hr.is_err() { println!("{}", serde_json::to_string(&Out{canceled:true,paths:vec![]}).unwrap()); CoUninitialize(); return Ok(()); }

  let mut out = Out{canceled:false, paths:vec![]};
  if multi {
    let arr = dlg.GetResults()?;
    let count = arr.GetCount()?;
    for i in 0..count {
      let si = arr.GetItemAt(i)?;
      out.paths.push(shellitem_to_path(&si)?);
    }
  } else {
    let si = dlg.GetResult()?;
    out.paths.push(shellitem_to_path(&si)?);
  }
  println!("{}", serde_json::to_string(&out).unwrap());
  CoUninitialize();
  Ok(())
}}

unsafe fn shellitem_to_path(si:&IShellItem)->Result<String>{
  let pw = si.GetDisplayName(SIGDN_FILESYSPATH)?;
  let s = pw.to_string().unwrap_or_default();
  Ok(s)
}