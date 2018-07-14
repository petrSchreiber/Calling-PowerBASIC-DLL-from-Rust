#compile dll "..\pblib-test\target\release\pblib.dll"
#dim all

#include "WIN32API.INC"

' General advice:
' - Use 'alias' to define procedure name in DLL
' - Use 'cdecl' to prevent esoteric behaviour
' - Use 'export' to make the function visible in DLL

sub println(byVal message as string)
  local hOutput, bytesWritten as dword
  message += chr$(10)
  hOutput = GetStdHandle(%STD_OUTPUT_HANDLE)
  WriteFile hOutput, byVal strptr(message), len(message), byVal varPtr(bytesWritten), byVal %NULL
end sub

' Functions and procedures exposed to DLL user follow
#include "unsigned_integers.inc"  ' BYTE, WORD, DWORD
#include "signed_integers.inc"    ' INTEGER, LONG, QUAD
#include "floating_points.inc"    ' SINGLE, DOUBLE

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
