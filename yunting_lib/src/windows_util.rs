/*
Copyright (c) 2022 masinc <masinc000@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */
use std::{ffi::OsString, os::windows::prelude::OsStringExt, path::PathBuf};

use windows_sys::Win32::{
    Foundation::MAX_PATH,
    UI::Shell::{self, SHGetFolderPathW},
};

/// The `SpecialFolder` enum represents the special folders on Windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(unused)]
pub enum SpecialFolder {
    /// The folder for the user's administrative tools.
    AdminTools = Shell::CSIDL_ADMINTOOLS as isize,
    /// The folder for the user's application data.
    ApplicationData = Shell::CSIDL_APPDATA as isize,
    /// The folder for the user's recycle bin.
    RecycleBin = Shell::CSIDL_BITBUCKET as isize,
    /// The folder for the user's CD burning area.
    CDBurning = Shell::CSIDL_CDBURN_AREA as isize,

    /// The folder for the user's common application data.
    CommonAdminTools = Shell::CSIDL_COMMON_ADMINTOOLS as isize,
    /// The folder for the user's common application data.
    CommonAppData = Shell::CSIDL_COMMON_APPDATA as isize,
    /// The folder for the user's common desktop.
    CommonDesktopDirectory = Shell::CSIDL_COMMON_DESKTOPDIRECTORY as isize,
    /// The folder for the user's common documents.
    CommonDocuments = Shell::CSIDL_COMMON_DOCUMENTS as isize,
    /// The folder for the user's common favorites.
    CommonFavorites = Shell::CSIDL_COMMON_FAVORITES as isize,
    /// The folder for the user's common music.
    CommonMusic = Shell::CSIDL_COMMON_MUSIC as isize,
    /// The folder for the user's common oem links.
    CommonOemLinks = Shell::CSIDL_COMMON_OEM_LINKS as isize,
    /// The folder for the user's common pictures.
    CommonPictures = Shell::CSIDL_COMMON_PICTURES as isize,
    /// The folder for the user's common programs.
    CommonPrograms = Shell::CSIDL_COMMON_PROGRAMS as isize,
    /// The folder for the user's common start menu.
    CommonStartMenu = Shell::CSIDL_COMMON_STARTMENU as isize,
    /// The folder for the user's common startup.
    CommonStartup = Shell::CSIDL_COMMON_STARTUP as isize,
    /// The folder for the user's common templates.
    CommonTemplates = Shell::CSIDL_COMMON_TEMPLATES as isize,
    /// The folder for the user's common video.
    CommonVideos = Shell::CSIDL_COMMON_VIDEO as isize,

    /// The folder for the user's computers near me.
    NetworkFolder = Shell::CSIDL_COMPUTERSNEARME as isize,
    /// The folder for the user's network connections.
    Connections = Shell::CSIDL_CONNECTIONS as isize,

    /// The folder for the user's control panel.
    ControlPanel = Shell::CSIDL_CONTROLS as isize,

    /// The folder for the user's cookies.
    Cookies = Shell::CSIDL_COOKIES as isize,
    /// The folder for the user's desktop.
    Desktop = Shell::CSIDL_DESKTOP as isize,
    /// The folder for the user's desktop.
    DesktopDirectory = Shell::CSIDL_DESKTOPDIRECTORY as isize,
    /// The folder for the user's drives.
    Drives = Shell::CSIDL_DRIVES as isize,
    /// The folder for the user's favorites.
    Favorites = Shell::CSIDL_FAVORITES as isize,
    /// The folder for the user's fonts.
    Fonts = Shell::CSIDL_FONTS as isize,
    /// The folder for the user's history.
    History = Shell::CSIDL_HISTORY as isize,
    /// The folder for the user's internet.
    Internet = Shell::CSIDL_INTERNET as isize,
    /// The folder for the user's internet cache.
    InternetCache = Shell::CSIDL_INTERNET_CACHE as isize,
    /// The folder for the user's local application data.
    LocalApplicationData = Shell::CSIDL_LOCAL_APPDATA as isize,
    /// The folder for the user's local documents.
    MyDocuments = Shell::CSIDL_MYDOCUMENTS as isize,
    /// The folder for the user's local music.
    MyMusic = Shell::CSIDL_MYMUSIC as isize,
    /// The folder for the user's local pictures.
    MyPictures = Shell::CSIDL_MYPICTURES as isize,
    /// The folder for the user's local video.
    MyVideos = Shell::CSIDL_MYVIDEO as isize,
    /// The folder for the user's network.
    NetHood = Shell::CSIDL_NETHOOD as isize,
    /// The folder for the user's network shortcut.
    NetworkShortcuts = Shell::CSIDL_NETWORK as isize,
    /// The folder for the user's printer shortcut.
    PrinterShortcuts = Shell::CSIDL_PRINTERS as isize,

    /// The folder for the user's prgram files.
    ProgramFiles = Shell::CSIDL_PROGRAM_FILES as isize,
    /// The folder for the user's program files (x86).
    ProgramFilesX86 = Shell::CSIDL_PROGRAM_FILESX86 as isize,

    /// The folder for the user's common program files.
    CommonProgramFiles = Shell::CSIDL_PROGRAM_FILES_COMMON as isize,
    /// The folder for the user's common program files (x86).
    CommonProgramFilesX86 = Shell::CSIDL_PROGRAM_FILES_COMMONX86 as isize,

    /// The folder for the user's programs.
    Programs = Shell::CSIDL_PROGRAMS as isize,
    /// The folder for the user's recent.
    Recent = Shell::CSIDL_RECENT as isize,
    /// The folder for the user's resources.
    Resource = Shell::CSIDL_RESOURCES as isize,
    /// The folder for the user's send to.
    SendTo = Shell::CSIDL_SENDTO as isize,
    /// The folder for the user's start menu.
    StartMenu = Shell::CSIDL_STARTMENU as isize,
    /// The folder for the user's startup.
    Startup = Shell::CSIDL_STARTUP as isize,
    /// The folder for the user's system.
    System = Shell::CSIDL_SYSTEM as isize,
    /// The folder for the user's system x86.
    SystemX86 = Shell::CSIDL_SYSTEMX86 as isize,
    /// The folder for the user's templates.
    Templates = Shell::CSIDL_TEMPLATES as isize,
    /// The folder for the user's profile.
    UserProfile = Shell::CSIDL_PROFILE as isize,
    /// The folder for the user's windows.
    Windows = Shell::CSIDL_WINDOWS as isize,
}

impl SpecialFolder {
    /// Get the path for a special folder.
    pub fn get(&self) -> Option<PathBuf> {
        let mut path = [0; MAX_PATH as usize];
        let result = unsafe {
            SHGetFolderPathW(
                std::ptr::null_mut(),
                *self as _,
                std::ptr::null_mut(),
                0,
                &mut path[0],
            )
        };

        if result == 0 {
            let p: Vec<u16> = path.into_iter().take_while(|&x| x != 0).collect();
            let s = OsString::from_wide(&p);
            Some(s.into())
        } else {
            None
        }
    }
}
