## Working with Starknet devnet


## 1. Environments

* Pyenv

* Scarb

* Starknet Foundry

## Install Starknet Foundry;

1. Install starknet foundry using a script;

```sh
hp@Cyndie:~$ curl -L https://raw.githubusercontent.com/foundry-rs/starknet-foundry/master/scripts/install.sh | sh
```

You should see this when successful;

```sh
Starknet Foundry has been successfully installed and should be already available in your PATH.
Run 'snforge --version' and 'sncast --version' to verify your installation. Happy coding!
hp@Cyndie:~$ snforge --version
forge 0.5.0
hp@Cyndie:~$ sncast --version
cast 0.5.0
```

## Create your project directory for cloning the starknet foundry template

```sh
hp@Cyndie:~/Desktop$ mkdir starknet-foundry-project
hp@Cyndie:~/Desktop$ git clone https://github.com/foundry-rs/starknet_forge_template.git
```


## Install Starknet Devnet;

1. First install starknet devnet using pip;

```sh
hp@Cyndie:~/web3_club/starknet_forge_template/devnet$ pip install starknet-devnet
```

2. Run starknet devnet with this command;

```sh
hp@Cyndie:~/web3_club/starknet_forge_template/devnet$ starknet-devnet

```
