# SIMPLE CAIRO VM WRITTEN IN RUST;

This repo is an implementation of a simple cairo vm for compiling Cairo language, written in rust.


# Introduction

Cairo is a unique language tailored for zk-STARKs and has applications in creating scalable and transparent computational integrity proofs. 

This project aims to provide a Rust-based environment for compiling and running Cairo programs.

# Features of the VM

The VM Consists of a ;

* ### Lexer : Tokenize Cairo Code

* ### Parser: Analyzes the tokenized output from the lexer and builds an Abstract Syntax Tree (AST).

* ### Type-Checker: Validates the types used in the Cairo programs to ensure correctness.

* ### Translator: Converts the AST into bytecode or machine code suitable for the VM.

* ### Runtime environment: Executes the translated code.


N.B extra features and complexity will be added to mimic starknet and cairo architcture properly.

The lexer is currently under construction. It's a manual lexer, for more efficiency the `logos` Rust crate will be used to build a more efficient and stable lexer.

# Clone the Repo

```sh
git clone https://github.com/CyndieKamau/cairovm_rust

cd cairovm_rust

cargo build

```

Here's an overview of how the current lexer is functioning;

```sh
hp@Cyndie:~/Desktop/cairovm/lexer/src$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `/home/hp/Desktop/cairovm/lexer/target/debug/lexer`
Please enter a simple cairo code here: 
let mut x = 5_u32
[Let, Mut, Identifier("x"), Assign, Number("5", None), Identifier("u32")]
```

# Deployment of Actix Web Server using DigitalOcean

## STEP 1: Create a Digital Ocean account

## STEP 2: Create a New Droplet:

* Choose a region (San Francisco, New York etc)
* Choose a Data Center within the region (San Francisco.Datacenter 3. SF03)
* Choose an image (Ubuntu 22.04 LTS x64 etc)
* Choose Size (Basic, General Purpose etc)
* Choose CPU options (Regular, Premium Intel etc)
* Choose SSH Authentication method :
  
    * To generate a new SSH key:
    * Run `ssh-keygen`
    * You will be prompted to save and name the key: 
  
        ```sh
        Generating public/private rsa key pair.
        Enter file in which to save the key
        (/Users/USER/.ssh/id_rsa):

        ```
    N.B: If you have setup ssh already with github rename the `id_rsa` file to avoid over-writing it, example `dig_id_rsa`

    * Create and confirm a passphrase for the key (highly recommended):
    
        ```sh
        Enter passphrase (empty for no passphrase):

        Enter same passphrase again:
        ```

    * This will generate two files: `dig_id_rsa` and `dig_id_rsa.pub`
    * Copy paste the `dig_id_rsa.pub` to the SSH Key content field in Digital Ocean:

    `cat ~/.ssh/dig_id_rsa.pub`

* Create Doplet
* Once done, ssh into your virtual machine using the IP address allocated by Digital Ocean:

    ```sh
    $ ssh root@your_ip_address
    The authenticity of host ' ()' can't be established.
    ECDSA key fingerprint is SHA256:glf52FoAchmTtbFsYDHXiigswi9LWtFYIya4eJtKDto.
    Are you sure you want to continue connecting (yes/no/[fingerprint])? yes
    Warning: Permanently added '' (ECDSA) to the list of known hosts.
    Welcome to Ubuntu 22.04.2 LTS (GNU/Linux 5.15.0-67-generic x86_64)
    ...
    ```


## STEP 3: Installing Dependencies in your VM

* The VM is in essence an empty Linux VM
* First update it: `sudo apt update && sudo apt upgrade`
* Install the dependencies needed to run the rust web server:
  
  * **Rust:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  * **Node:** 
    ```sh
    # install nvm first
    $ sudo apt install curl
    $ curl https://raw.githubusercontent.com/creationix/nvm/master/install.sh | bash 

    # Load your environment
    $  source ~/.bashrc

    # Install Node using nvm
    $ nvm install node 

    # If you want specific node version
    $ nvm install 18.16.0

    ```

   * **Nginx:** `sudo apt install nginx`
  
    Copy Paste your IP address to the browser.

    You should see such a message that confirms Nginx is properly installed:

        ```
        Welcome to nginx!

        If you see this page, the nginx web server is successfully installed and working
        ....
        ``` 

## STEP 4: Configure Nginx as a reverse proxy server

* Access the Nginx configuration:

    * Create a new config file in `/etc/nginx/sites-available/` directory:
  
    `sudo nano /etc/nginx/sites-available/myapp`

    Replace `myapp` with the name of your choice.

    * Create a symbolic link to the config file created to link it to `sites-enabled` directory:
  
        `sudo ln -s /etc/nginx/sites-available/myapp /etc/nginx/sites-enabled/`

    * Check if the Nginx connection is Okay:

        ```sh
        $ sudo nginx -t
        nginx: the configuration file /etc/nginx/nginx.conf syntax is ok
        nginx: configuration file /etc/nginx/nginx.conf test is successful
        ```

    **N.B.** If you get this error your symlink has not been configured properly:


        ```sh
        nginx: [emerg] open() "/etc/nginx/sites-enabled/cairovm" failed (40: Too many levels of symbolic links) in /etc/nginx/nginx.conf:60
        nginx: configuration file /etc/nginx/nginx.conf test failed

        ```

    * Configure the `myapp` file:

        ```
        server {

            server_name cairovm.supercodehive.com www.cairovm.supercodehive.com;

            location / {
                proxy_pass http://localhost:8080; # Point to the Actix web server
                proxy_buffering off;
                proxy_set_header X-Real-IP $remote_addr;
            }
        }

        ```
    Note that the server name should be your domain name.

    You should also specify your Actix web server port: `http://localhost:8080` 

* Configure firewall settings to allow traffic:

    ```sh
    $ sudo ufw allow 'Nginx Full'
    $ sudo ufw allow 'Nginx HTTP'
    ```

## STEP 5: Clone and Run your Rust project in the VM:

* Create a directory to hold your Rust project:

     `mkdir apps && cd apps`

* Clone your repo inside the `apps` dir, and build the dependencies:

    ```sh
    $ git clone https://github.com/CyndieKamau/cairovm_rust.git

    $ cd cairovm_rust

    $ cargo run
    ```

## STEP 6: Create systemd service to serve your web application

Create a systemd service for your web server

* Open a new service file:
  
  `sudo nano /etc/systemd/system/cairovm.service
`

  Rename the file to suit your project.

* Edit the `cairovm.service` file:


    ```
        [Unit]
        Description=Actix Web Application

        [Service]
        User=root
        Group=root
        WorkingDirectory=/path/to/your/rust/app/
        # Set any environment variables your app needs, e.g., database URLs
        Environment="RUST_LOG=info"
        ExecStart=/path/to/your/app/target/release/your_app_executable

        [Install]
        WantedBy=multi-user.target
    ```

    Add the appropriate user and group, based on your server configuration.

    To get the number of users and groups in your server, use `getent passwd`

    `WorkingDirectory` is the file path to your rust project

    `ExecStart` is the path to your rust binary after running the program. 

    It's `release` if you ran in production mode; for local development its `debug`.

* Restart the systemd daemon:

  `$ sudo systemctl daemon-reload`

* Enable `cairovm.service` to run automatically after every boot up:

  `$ sudo systemctl enable cairovm.service`

* Start `cairovm.service`:
  
  `sudo systemctl start cairovm.service`

* Successful configuration will be like this:

  ```sh
    $ sudo systemctl status cairovm.service
    ● cairovm.service - Actix Web Application
        Loaded: loaded (/etc/systemd/system/cairovm.service; enabled; vendor preset: enabled)
        Active: active (running) since Wed 2023-11-29 15:40:15 UTC; 3min 16s ago
    Main PID: 32594 (cairovm)
        Tasks: 3 (limit: 1116)
        Memory: 1.1M
            CPU: 133ms
        CGroup: /system.slice/cairovm.service
                └─32594 /root/apps/cairovm_rust/target/debug/cairovm

    Nov 29 15:40:15 ubuntu-22-04 cairovm[32594]: [2023-11-29T15:40:15Z INFO  actix_server::builder] starting 1 workers
    Nov 29 15:40:15 ubuntu-22-04 systemd[1]: Started Actix Web Application.
    Nov 29 15:40:15 ubuntu-22-04 cairovm[32594]: [2023-11-29T15:40:15Z INFO  actix_server::server] Actix runtime found; starting in Actix 
  ```

* Incase you get this error, ensure while writing the path to your `ExecStart` avoid including `/` at the end as the executable is treated as a directory:

    ```sh
        Failed to restart cairovm.service: Unit cairovm.service has a bad unit file setting.
    See system logs and 'systemctl status cairovm.service' for details.
    root@ubuntu-22-04:~/apps/cairovm_rust/target/debug# systemctl status cairovm.service
    ○ cairovm.service - Actix Web Application
        Loaded: bad-setting (Reason: Unit cairovm.service has a bad unit file setting.)
        Active: inactive (dead)

    Nov 29 15:23:31 ubuntu-22-04 systemd[1]: /etc/systemd/system/cairovm.service:10: Executable path specifies a directory: /root/apps/cai>
    Nov 29 15:23:31 ubuntu-22-04 systemd[1]: cairovm.service: Unit configuration has fatal error, unit will not be started.
    Nov 29 15:24:24 ubuntu-22-04 systemd[1]: /etc/systemd/system/cairovm.service:10: Executable path specifies a directory: /root/apps/cai>
    Nov 29 15:24:24 ubuntu-22-04 systemd[1]: cairovm.service: Unit configuration has fatal error, unit will not be started.
    Nov 29 15:29:55 ubuntu-22-04 systemd[1]: /etc/systemd/system/cairovm.service:10: Executable path specifies a directory: /root/apps/cai>
    Nov 29 15:29:55 ubuntu-22-04 systemd[1]: cairovm.service: Unit configuration has fatal error, unit will not be started.
    ...skipping...
    ○ cairovm.service - Actix Web Application
        Loaded: bad-setting (Reason: Unit cairovm.service has a bad unit file setting.)
        Active: inactive (dead)

    ```


## STEP 7: Enable Https for your website

* Install `certbot`:

    ```sh
    $ sudo snap install --classic certbot

    $ sudo ln -s /snap/bin/certbot /usr/bin/certbot
    ```

* Enable Https using `certbot`:

    ```sh
    $ sudo certbot --nginx

    ....
    Successfully received certificate.
    Certificate is saved at: /etc/letsencrypt/live/cairovm.supercodehive.com/fullchain.pem
    Key is saved at:         /etc/letsencrypt/live/cairovm.supercodehive.com/privkey.pem
    This certificate expires on 2024-02-27.
    These files will be updated when the certificate renews.
    Certbot has set up a scheduled task to automatically renew this certificate in the background.

    Deploying certificate
    Successfully deployed certificate for cairovm.supercodehive.com to /etc/nginx/sites-enabled/cairovm
    Successfully deployed certificate for www.cairovm.supercodehive.com to /etc/nginx/sites-enabled/cairovm
    Congratulations! You have successfully enabled HTTPS on https://cairovm.supercodehive.com and https://www.cairovm.supercodehive.com

    ```

* Renew the generated certificates:

    ```sh
    $ sudo certbot renew --dry-run

    ....
    Congratulations, all simulated renewals succeeded: 
  /etc/letsencrypt/live/cairovm.supercodehive.com/fullchain.pem (success)
    ```












