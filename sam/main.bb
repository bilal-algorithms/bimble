ON main(){
    may name = "JOYJOY_GANG";
    echoln(name);
    greet();
}
ON greet(){
    echoln("nice to meet you ",name);
    may urname = "1";
    takein(urname);
    echoln("");
    echo("nice to meet you too ",urname);
    out.flush();
}