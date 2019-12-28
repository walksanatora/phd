<!--
      /       |
 ___ (___  ___|
|   )|   )|   )
|__/ |  / |__/
|
--> <p align="center"><img src="./img/logo2.png"></p>

`phd` is an esoteric gopher server for small gopher sites.

point it at a directory and it'll serve up all its text files, sub-directories, and binary files over gopher. executable files will be run, like cgi! 

special files:

- **header.gph**: if it exists in a directory, its content will be shown above the directory's content. put ascii art in it.
- **footer.gph**: same, but will be shown below a directory's content.
- **index.gph**: completely replaces a directory's content with what's in this file.
- **??.gph**: visiting gopher://yoursite/1/dog/ will try to render `dog.gph` from disk.
- **.reverse**: if this exists, the directory contents will be listed in reverse alphanumeric order. useful for phloggin'.

any line in a `.gph` file that doesn't contain tabs (`\t`) and doesn't start with an `i` will get an `i` automatically prefixed, turning it into a gopher information item. 

any `.gph` file that is marked **executable** with be run as if it were a shell script and its output will be sent to the client. it will be passed three arguments: the query string (if any, the host, and the port. do with them what you will. 

for example:

    $ cat echo.gph
    #!/bin/sh
    echo "Hi, world! You said:" $1
    echo "1Visit Gopherpedia	/	gopherpedia.com	70"


then:

    $ gopher-client gopher://localhost/1/echo?something
    [INFO] Hi, world! You said: something
    [LINK] Visit Gopherpedia

or more seriously:

    $ cat figlet.gph
    #!/bin/sh
    figlet $1

then:

    $ gopher-client gopher://localhost/1/figlet?hi gopher
    [INFO]  _     _                     _               
    [INFO] | |__ (_)   __ _  ___  _ __ | |__   ___ _ __ 
    [INFO] | '_ \| |  / _` |/ _ \| '_ \| '_ \ / _ \ '__|
    [INFO] | | | | | | (_| | (_) | |_) | | | |  __/ |   
    [INFO] |_| |_|_|  \__, |\___/| .__/|_| |_|\___|_|   
    [INFO]             |___/      |_|                    


## usage

    phd [options] <directory>

    phd ./path/to/gopher/root    # Serve directory over port 70.
    phd -p 7070 docs             # Serve 'docs' directory on port 7070
    phd -h localhost             # Serve cwd using hostname "localhost".

## development

    cargo run -- ./path/to/gopher/site

## resources

- https://github.com/gophernicus/gophernicus/blob/master/README.Gophermap
- https://gopher.zone/posts/how-to-gophermap/
- [rfc 1436](https://tools.ietf.org/html/rfc1436)

## todo

- [ ] script mode
- [ ] log options
- [ ] 404 message
