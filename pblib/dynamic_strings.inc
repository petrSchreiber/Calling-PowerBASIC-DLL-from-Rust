' STRING
sub passing_string_byval alias "passing_string_byval" (byval s as string) export
  println using$("  PowerBASIC: Text passed to &: &", FUNCNAME$, s)
end sub

sub passing_string_byref alias "passing_string_byref" (byref s as string) export
  println using$("  PowerBASIC: Text passed to &: &", FUNCNAME$, s)
end sub

sub passing_string_byref_with_change alias "passing_string_byref_with_change" (byref s as string) export
  println using$("  PowerBASIC: Text passed to &: &", FUNCNAME$, s)
  s = "Ciao, Rustito!"
  println using$("  PowerBASIC: Text passed to & changed to &", FUNCNAME$, s)
end sub
