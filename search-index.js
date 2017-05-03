var searchIndex = {};
searchIndex["wiki"] = {"doc":"The executable wiki for wikilib","items":[[0,"error","wiki","Everything related to the wikilib error handling",null,null],[3,"Error","wiki::error","The Error type.",null,null],[12,"0","","The kind of the error.",0,null],[4,"ErrorKind","","The kind of an error.",null,null],[13,"Msg","","A convenient variant for String.",1,null],[13,"Io","","An I/O error",1,null],[13,"Glob","","A glob error",1,null],[13,"Pattern","","A glob pattern error",1,null],[13,"Http","","A http error",1,null],[6,"Result","","Convenient wrapper around `std::Result`.",null,null],[8,"ResultExt","","Additional methods for `Result`, for easy interaction with this crate.",null,null],[10,"chain_err","","If the `Result` is an `Err` then `chain_err` evaluates the closure, which returns *some type that can be converted to `ErrorKind`*, boxes the original error to store as the cause, then returns a new error containing the original error.",2,{"inputs":[{"name":"self"},{"name":"f"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",0,{"inputs":[{"name":"errorkind"},{"name":"state"}],"output":{"name":"error"}}],[11,"from_kind","","",0,null],[11,"with_chain","","",0,{"inputs":[{"name":"e"},{"name":"k"}],"output":{"name":"self"}}],[11,"kind","","",0,null],[11,"iter","","",0,{"inputs":[{"name":"self"}],"output":{"name":"errorchainiter"}}],[11,"backtrace","","",0,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"extract_backtrace","","",0,{"inputs":[{"name":"error"}],"output":{"name":"option"}}],[11,"from_kind","","Constructs an error from a kind, and generates a backtrace.",0,{"inputs":[{"name":"errorkind"}],"output":{"name":"error"}}],[11,"with_chain","","Constructs a chained error from another error and a kind, and generates a backtrace.",0,{"inputs":[{"name":"e"},{"name":"k"}],"output":{"name":"error"}}],[11,"kind","","Returns the kind of the error.",0,{"inputs":[{"name":"self"}],"output":{"name":"errorkind"}}],[11,"iter","","Iterates over the error chain.",0,{"inputs":[{"name":"self"}],"output":{"name":"errorchainiter"}}],[11,"backtrace","","Returns the backtrace associated with this error.",0,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"description","","",0,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"cause","","",0,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",0,{"inputs":[{"name":"error"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"globerror"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"patternerror"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"httperror"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"errorkind"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"str"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"string"}],"output":{"name":"self"}}],[11,"deref","","",0,null],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","A string describing the error kind.",1,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"from","","",1,{"inputs":[{"name":"str"}],"output":{"name":"self"}}],[11,"from","","",1,{"inputs":[{"name":"string"}],"output":{"name":"self"}}],[11,"from","","",1,{"inputs":[{"name":"error"}],"output":{"name":"self"}}]],"paths":[[3,"Error"],[4,"ErrorKind"],[8,"ResultExt"]]};
searchIndex["wikilib"] = {"doc":"The lib for markdown based static HTML wiki generation","items":[[3,"Wiki","wikilib","Global processing structure",null,null],[0,"error","","Everything related to the wikilib error handling",null,null],[3,"Error","wikilib::error","The Error type.",null,null],[12,"0","","The kind of the error.",0,null],[4,"ErrorKind","","The kind of an error.",null,null],[13,"Msg","","A convenient variant for String.",1,null],[13,"Io","","An I/O error",1,null],[13,"Glob","","A glob error",1,null],[13,"Pattern","","A glob pattern error",1,null],[13,"Http","","A http error",1,null],[6,"Result","","Convenient wrapper around `std::Result`.",null,null],[8,"ResultExt","","Additional methods for `Result`, for easy interaction with this crate.",null,null],[10,"chain_err","","If the `Result` is an `Err` then `chain_err` evaluates the closure, which returns *some type that can be converted to `ErrorKind`*, boxes the original error to store as the cause, then returns a new error containing the original error.",2,{"inputs":[{"name":"self"},{"name":"f"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",0,{"inputs":[{"name":"errorkind"},{"name":"state"}],"output":{"name":"error"}}],[11,"from_kind","","",0,null],[11,"with_chain","","",0,{"inputs":[{"name":"e"},{"name":"k"}],"output":{"name":"self"}}],[11,"kind","","",0,null],[11,"iter","","",0,{"inputs":[{"name":"self"}],"output":{"name":"errorchainiter"}}],[11,"backtrace","","",0,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"extract_backtrace","","",0,{"inputs":[{"name":"error"}],"output":{"name":"option"}}],[11,"from_kind","","Constructs an error from a kind, and generates a backtrace.",0,{"inputs":[{"name":"errorkind"}],"output":{"name":"error"}}],[11,"with_chain","","Constructs a chained error from another error and a kind, and generates a backtrace.",0,{"inputs":[{"name":"e"},{"name":"k"}],"output":{"name":"error"}}],[11,"kind","","Returns the kind of the error.",0,{"inputs":[{"name":"self"}],"output":{"name":"errorkind"}}],[11,"iter","","Iterates over the error chain.",0,{"inputs":[{"name":"self"}],"output":{"name":"errorchainiter"}}],[11,"backtrace","","Returns the backtrace associated with this error.",0,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"description","","",0,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"cause","","",0,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",0,{"inputs":[{"name":"error"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"globerror"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"patternerror"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"httperror"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"errorkind"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"str"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"string"}],"output":{"name":"self"}}],[11,"deref","","",0,null],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","A string describing the error kind.",1,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"from","","",1,{"inputs":[{"name":"str"}],"output":{"name":"self"}}],[11,"from","","",1,{"inputs":[{"name":"string"}],"output":{"name":"self"}}],[11,"from","","",1,{"inputs":[{"name":"error"}],"output":{"name":"self"}}],[11,"default","wikilib","",3,{"inputs":[],"output":{"name":"wiki"}}],[11,"new","","Create a new `Wiki` instance",3,{"inputs":[],"output":{"name":"self"}}],[11,"init_logging","","Creates a new instance of the processing lib",3,{"inputs":[{"name":"self"},{"name":"loglevel"}],"output":{"name":"result"}}],[11,"read_from_directory","","Reads all markdown files recursively from a given directory. Clears the current available paths",3,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"result"}}],[11,"list_current_paths","","Print absolute path of all added md files",3,{"inputs":[{"name":"self"}],"output":null}],[11,"read_content_from_current_paths","","Read the content of all files and convert it to HTML",3,{"inputs":[{"name":"self"},{"name":"str"},{"name":"str"}],"output":{"name":"result"}}],[11,"serve","","Create an HTTP server serving the generated files",3,{"inputs":[{"name":"self"},{"name":"str"}],"output":{"name":"result"}}]],"paths":[[3,"Error"],[4,"ErrorKind"],[8,"ResultExt"],[3,"Wiki"]]};
initSearch(searchIndex);
