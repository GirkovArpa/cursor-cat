#NoTrayIcon

SetTitleMatchMode 3
WinSet, ExStyle, ^0x20, Cursor Cat
RemoveTaskbarButton(WinExist("A"))
return

RemoveTaskbarButton(activeHwnd) {
	;http://www.autohotkey.com/board/topic/83159-solved-removing-windows-taskbar-icons/#entry529572
	IID_ITaskbarList  := "{56FDF342-FD6D-11d0-958A-006097C9A090}"
	CLSID_TaskbarList := "{56FDF344-FD6D-11d0-958A-006097C9A090}"
	tbl := ComObjCreate(CLSID_TaskbarList, IID_ITaskbarList)
	DllCall(NumGet(NumGet(tbl+0), 3*A_PtrSize), "ptr", tbl)
	DllCall(NumGet(NumGet(tbl+0), 5*A_PtrSize), "ptr", tbl, "ptr", activeHwnd)
	ObjRelease(tbl) 
}