import { Title, Text, Anchor, Grid, Image, Stack } from '@mantine/core';
import classes from './Welcome.module.css';
import FloatingImage from '../others/FloatingImage';

export function Welcome() {
  return (
    <>
      <Stack>
        <Image w={'200px'} mx={'auto'} src={'/images/logo.png'} />
        <Title className={classes.title} ta="center" mt={50}>
          Welcome to{' '}
          <Text inherit variant="gradient" component="span" gradient={{ from: 'pink', to: 'yellow' }}>
            Cairo VM
          </Text>
        </Title>
        <Text c="dimmed" ta="center" size="lg" maw={580} mx="auto" mt="xl">
          (Write intro message)
        </Text>
      </Stack>
    </>
  );
}
