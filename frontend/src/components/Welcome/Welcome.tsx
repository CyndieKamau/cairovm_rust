import { Title, Text, Anchor, Grid, Image, Stack } from '@mantine/core';
import classes from './Welcome.module.css';
import FloatingImage from '../others/FloatingImage';

export function Welcome() {
  return (
    <>
      <Stack>
        <Image w={'200px'} mx={'auto'} src={'/images/logo.png'} />
        <Title className={classes.title} ta="center" mt={50}>
          Welcome to the{' '}
          <Text
            inherit
            variant="gradient"
            component="span"
            gradient={{ from: 'pink', to: 'yellow' }}
          >
            Cairo VM
          </Text>
        </Title>
        <Text  ta="center" size="xl" maw={580} mx="auto" mt="xl">
          Cairo is a unique language tailored for zk-STARKs.
        </Text>

        <Text  ta="center" size="xl" maw={580} mx="auto" mt="xl">
          It has applications in creating scalable and transparent computational integrity proofs.
        </Text>

        <Text  ta="center" size="xl" maw={580} mx="auto" mt="xl">
          Cairo VM (Virtual Machine) aims to provide a Rust-based environment for compiling and
          running Cairo programs.
        </Text>

        <Text  ta="center" size="xl" maw={580} mx="auto" mt="xl">
          Below is the lexer section of the compiler, for tokenizing the Cairo code.

          Paste your sample code below:
        </Text>


      </Stack>
    </>
  );
}
