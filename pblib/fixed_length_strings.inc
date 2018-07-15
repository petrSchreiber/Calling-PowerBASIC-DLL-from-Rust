' STRINGZ = ASCIIZ
sub passing_stringz alias "passing_stringz" (sz as stringz) export cdecl
  println using$("  PowerBASIC: Text passed to &: &", FUNCNAME$, sz)
end sub

sub passing_stringz_ptr alias "passing_stringz_ptr" (byVal szp as stringz ptr) export cdecl
  println using$("  PowerBASIC: Text passed to &: &", FUNCNAME$, @szp)
end sub

sub passing_stringz_ptr_with_change alias "passing_stringz_ptr_with_change" (byVal szp as stringz ptr) export cdecl
  println using$("  PowerBASIC: Text passed to &: &", FUNCNAME$, @szp)
  @szp = "Hi, Rust!"
  println using$("  PowerBASIC: Value passed to & changed to: &", FUNCNAME$, @szp)
end sub
