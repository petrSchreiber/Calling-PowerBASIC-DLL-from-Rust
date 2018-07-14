' SINGLE
sub passing_single_byval alias "passing_single_byval" (byVal n as single) export cdecl
  println "  PowerBASIC: Value passed to passing_single_byval: " + format$(n)
end sub

sub passing_single_byref alias "passing_single_byref" (byRef n as single) export cdecl
  println "  PowerBASIC: Value passed to passing_single_byref: " + format$(n)
end sub

sub passing_single_byref_with_change alias "passing_single_byref_with_change" (byRef n as single) export cdecl
  println "  PowerBASIC: Value passed to passing_single_byref_with_change: " + format$(n)
  n = 2.222222
  println "  PowerBASIC: Value changed in passing_single_byref_with_change to: " + format$(n)
end sub

' DOUBLE
sub passing_double_byval alias "passing_double_byval" (byVal n as double) export cdecl
  println "  PowerBASIC: Value passed to passing_double_byval: " + format$(n)
end sub

sub passing_double_byref alias "passing_double_byref" (byRef n as double) export cdecl
  println "  PowerBASIC: Value passed to passing_double_byref: " + format$(n)
end sub

sub passing_double_byref_with_change alias "passing_double_byref_with_change" (byRef n as double) export cdecl
  println "  PowerBASIC: Value passed to passing_double_byref_with_change: " + format$(n)
  n = 2.222222222222
  println "  PowerBASIC: Value changed in passing_double_byref_with_change to: " + format$(n)
end sub

' EXT
' not supported in Rust; however, Rust has 128bit floating point not supported in PowerBASIC
