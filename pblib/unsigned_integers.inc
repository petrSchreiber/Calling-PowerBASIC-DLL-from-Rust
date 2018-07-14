' BYTE
sub passing_byte_byval alias "passing_byte_byval" (byVal n as byte) export cdecl
  println "  PowerBASIC: Value passed to passing_byte_byval: " + format$(n)
end sub

sub passing_byte_byref alias "passing_byte_byref" (byRef n as byte) export cdecl
  println "  PowerBASIC: Value passed to passing_byte_byref: " + format$(n)
end sub

sub passing_byte_byref_with_change alias "passing_byte_byref_with_change" (byRef n as byte) export cdecl
  println "  PowerBASIC: Value passed to passing_byte_byref_with_change: " + format$(n)
  n = 123
  println "  PowerBASIC: Value changed in passing_byte_byref_with_change to: " + format$(n)
end sub

' WORD
sub passing_word_byval alias "passing_word_byval" (byVal n as word) export cdecl
  println "  PowerBASIC: Value passed to passing_word_byval: " + format$(n)
end sub

sub passing_word_byref alias "passing_word_byref" (byRef n as word) export cdecl
  println "  PowerBASIC: Value passed to passing_word_byref: " + format$(n)
end sub

sub passing_word_byref_with_change alias "passing_word_byref_with_change" (byRef n as word) export cdecl
  println "  PowerBASIC: Value passed to passing_word_byref_with_change: " + format$(n)
  n = 42000
  println "  PowerBASIC: Value changed in passing_word_byref_with_change to: " + format$(n)
end sub

' DWORD
sub passing_dword_byval alias "passing_dword_byval" (byVal n as dword) export cdecl
  println "  PowerBASIC: Value passed to passing_dword_byval: " + format$(n)
end sub

sub passing_dword_byref alias "passing_dword_byref" (byRef n as dword) export cdecl
  println "  PowerBASIC: Value passed to passing_dword_byref: " + format$(n)
end sub

sub passing_dword_byref_with_change alias "passing_dword_byref_with_change" (byRef n as dword) export cdecl
  println "  PowerBASIC: Value passed to passing_dword_byref_with_change: " + format$(n)
  n = 4000000
  println "  PowerBASIC: Value changed in passing_dword_byref_with_change to: " + format$(n)
end sub
