# springwood-CLI
Springwood is a work in progress command line interface for the [GTD Protocol](https://hamberg.no/gtd) written in Rust.

![showcase](images/example.gif)

Springwood started as a personal project to help myself grasp a better understanding of Rust but I will work on it in order to potentially make a fully fleshed out releasable version. 

At the moment, it is in a very basic form and simply displays information from a JSON file. If you would still like to install it run: 

    mkdir ~/.springwood && cd ~/.springwood
    git clone https://github.com/Narfee/Springwood-CLI.git
    mv Springwood-CLI/* . && rm -rf Springwood-CLI
    cargo install --path ~/.springwood
   
 at the moment you will have to run springwood from its `.springwood` directory as a result of the data that it reads from the json file.

When springwood is ready for release it's repo will be updated and an official release will be made!
