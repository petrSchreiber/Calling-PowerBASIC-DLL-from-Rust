#compile dll "..\pblib-test\target\release\pblib.dll"
#dim all

#include "WIN32API.INC"

' General advice:
' - Use 'alias' to define procedure name in DLL
' - Use 'cdecl' to prevent esoteric behaviour
' - Use 'export' to make the function visible in DLL

' Functions and procedures exposed to DLL user follow

sub passing_byte_byval alias "passing_byte_byval" (byVal n as byte) export cdecl
  msgbox "Value passed to passing_byte_byval: " + format$(n)
end sub

sub passing_byte_byref alias "passing_byte_byref" (byRef n as byte) export cdecl
  msgbox "Value passed to passing_byte_byref: " + format$(n)
end sub

sub passing_byte_byref_with_change alias "passing_byte_byref_with_change" (byRef n as byte) export cdecl
  msgbox "Value passed to passing_byte_byref2: " + format$(n)
  n = 123
  msgbox "Value changed in passing_byte_byref2 to: " + format$(n)
end sub

' Needed for proper loading/unloading
function libmain alias "LibMain" (byVal hInstance   as long, _
                                  byVal fwdReason   as long, _
                                  byVal lpvReserved as long) export as long
  static hWin as dword

  select case fwdReason
    case %DLL_PROCESS_ATTACH

      function = 1
      exit function
    case %DLL_PROCESS_DETACH

      function = 1
      exit function
    case %DLL_THREAD_ATTACH

      function = 1
      exit function
    case %DLL_THREAD_DETACH

      function = 1
      exit function
  end select

end function
