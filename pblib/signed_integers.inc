' INTEGER
sub passing_integer_byval alias "passing_integer_byval" (byVal n as integer) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_integer_byref alias "passing_integer_byref" (byRef n as integer) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_integer_byref_with_change alias "passing_integer_byref_with_change" (byRef n as integer) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
  n = 24000
  println using$("  PowerBASIC: Value passed to & changed to: &", FUNCNAME$, format$(n))
end sub

function returning_integer alias "returning_integer" () export cdecl as integer
  dim n as integer
  n = 24123
  println using$("  PowerBASIC: Returning value from &: &", FUNCNAME$, format$(n))

  function = n
end function

' LONG
sub passing_long_byval alias "passing_long_byval" (byVal n as long) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_long_byref alias "passing_long_byref" (byRef n as long) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_long_byref_with_change alias "passing_long_byref_with_change" (byRef n as long) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
  n = 2000000000
  println using$("  PowerBASIC: Value passed to & changed to: &", FUNCNAME$, format$(n))
end sub

function returning_long alias "returning_long" () export cdecl as long
  dim n as long
  n = 2123456789
  println using$("  PowerBASIC: Returning value from &: &", FUNCNAME$, format$(n))

  function = n
end function

' QUAD
sub passing_quad_byval alias "passing_quad_byval" (byVal n as quad) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_quad_byref alias "passing_quad_byref" (byRef n as quad) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
end sub

sub passing_quad_byref_with_change alias "passing_quad_byref_with_change" (byRef n as quad) export cdecl
  println using$("  PowerBASIC: Value passed to &: &", FUNCNAME$, format$(n))
  n = 5000000000
  println using$("  PowerBASIC: Value passed to & changed to: &", FUNCNAME$, format$(n))
end sub

function returning_quad alias "returning_quad" () export cdecl as quad
  dim n as quad
  n = 5123456789
  println using$("  PowerBASIC: Returning value from &: &", FUNCNAME$, format$(n))

  function = n
end function
