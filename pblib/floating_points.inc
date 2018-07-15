' SINGLE
sub passing_single_byval alias "passing_single_byval" (byVal n as single) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_single_byref alias "passing_single_byref" (byRef n as single) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_single_byref_with_change alias "passing_single_byref_with_change" (byRef n as single) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
  n = 2.222222
  println using$("  PowerBASIC: Value passed to & changed to: &", FUNCNAME$, format$(n))
end sub

function returning_single alias "returning_single" () export cdecl as single
  dim n as single
  n = 3.333333
  println using$("  PowerBASIC: Returning value from &: &", FUNCNAME$, format$(n))

  function = n
end function

' DOUBLE
sub passing_double_byval alias "passing_double_byval" (byVal n as double) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_double_byref alias "passing_double_byref" (byRef n as double) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_double_byref_with_change alias "passing_double_byref_with_change" (byRef n as double) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
  n = 2.222222222222
  println using$("  PowerBASIC: Value passed to & changed to: &", FUNCNAME$, format$(n))
end sub

function returning_double alias "returning_double" () export cdecl as double
  dim n as double
  n = 3.333333333333
  println using$("  PowerBASIC: Returning value from &: &", FUNCNAME$, format$(n))

  function = n
end function

' EXT
' not supported in Rust; however, Rust has 128bit floating point not supported in PowerBASIC
