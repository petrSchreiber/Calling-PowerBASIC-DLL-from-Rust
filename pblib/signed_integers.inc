' INTEGER
sub passing_integer_byval alias "passing_integer_byval" (byVal n as integer) export cdecl
  println "  PowerBASIC: Value passed to passing_integer_byval: " + format$(n)
end sub

sub passing_integer_byref alias "passing_integer_byref" (byRef n as integer) export cdecl
  println "  PowerBASIC: Value passed to passing_integer_byref: " + format$(n)
end sub

sub passing_integer_byref_with_change alias "passing_integer_byref_with_change" (byRef n as integer) export cdecl
  println "  PowerBASIC: Value passed to passing_integer_byref_with_change: " + format$(n)
  n = 24000
  println "  PowerBASIC: Value changed in passing_integer_byref_with_change to: " + format$(n)
end sub

' LONG
sub passing_long_byval alias "passing_long_byval" (byVal n as long) export cdecl
  println "  PowerBASIC: Value passed to passing_long_byval: " + format$(n)
end sub

sub passing_long_byref alias "passing_long_byref" (byRef n as long) export cdecl
  println "  PowerBASIC: Value passed to passing_long_byref: " + format$(n)
end sub

sub passing_long_byref_with_change alias "passing_long_byref_with_change" (byRef n as long) export cdecl
  println "  PowerBASIC: Value passed to passing_long_byref_with_change: " + format$(n)
  n = 2000000000
  println "  PowerBASIC: Value changed in passing_long_byref_with_change to: " + format$(n)
end sub

' QUAD
sub passing_quad_byval alias "passing_quad_byval" (byVal n as quad) export cdecl
  println "  PowerBASIC: Value passed to passing_quad_byval: " + format$(n)
end sub

sub passing_quad_byref alias "passing_quad_byref" (byRef n as quad) export cdecl
  println "  PowerBASIC: Value passed to passing_quad_byref: " + format$(n)
end sub

sub passing_quad_byref_with_change alias "passing_quad_byref_with_change" (byRef n as quad) export cdecl
  println "  PowerBASIC: Value passed to passing_quad_byref_with_change: " + format$(n)
  n = 5000000000
  println "  PowerBASIC: Value changed in passing_quad_byref_with_change to: " + format$(n)
end sub
