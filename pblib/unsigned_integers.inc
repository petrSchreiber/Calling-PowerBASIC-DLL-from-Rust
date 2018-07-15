' BYTE
sub passing_byte_byval alias "passing_byte_byval" (byVal n as byte) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_byte_byref alias "passing_byte_byref" (byRef n as byte) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_byte_byref_with_change alias "passing_byte_byref_with_change" (byRef n as byte) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
  n = 123
  println using$("  PowerBASIC: Value passed to & changed to: &", FUNCNAME$, format$(n))
end sub

function returning_byte alias "returning_byte" () export cdecl as byte
  dim n as byte
  n = 127
  println using$("  PowerBASIC: Returning value from &: &", FUNCNAME$, format$(n))

  function = n
end function

' WORD
sub passing_word_byval alias "passing_word_byval" (byVal n as word) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_word_byref alias "passing_word_byref" (byRef n as word) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_word_byref_with_change alias "passing_word_byref_with_change" (byRef n as word) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
  n = 42000
  println using$("  PowerBASIC: Value passed to & changed to: &", FUNCNAME$, format$(n))
end sub

function returning_word alias "returning_word" () export cdecl as word
  dim n as word
  n = 41234
  println using$("  PowerBASIC: Returning value from &: &", FUNCNAME$, format$(n))

  function = n
end function

' DWORD
sub passing_dword_byval alias "passing_dword_byval" (byVal n as dword) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_dword_byref alias "passing_dword_byref" (byRef n as dword) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_dword_byref_with_change alias "passing_dword_byref_with_change" (byRef n as dword) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
  n = 4000000
  println using$("  PowerBASIC: Value passed to & changed to: &", FUNCNAME$, format$(n))
end sub

function returning_dword alias "returning_dword" () export cdecl as dword
  dim n as dword
  n = 4123456
  println using$("  PowerBASIC: Returning value from &: &", FUNCNAME$, format$(n))

  function = n
end function
