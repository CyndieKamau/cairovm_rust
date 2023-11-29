import { AppShell, Group, Title, Container, Text } from '@mantine/core';
import React, { Children } from 'react';
import { ColorSchemeToggle } from '../components/ColorSchemeToggle/ColorSchemeToggle';
import FloatingImage from '@/components/others/FloatingImage';

interface ImainLayout {
  children: React.ReactNode;
}
const MainLayout = (props: ImainLayout) => {
  const { children } = props;
  return (
    <AppShell header={{ height: { base: '70px' } }}>
      <AppShell.Header withBorder={false}>
        <Container className="h-100" size={'xl'}>
          <Group className="h-100" align="center" justify='space-between'>
            <Text variant="gradient" fw={500} size='32px' component="span" gradient={{ from: 'pink', to: 'yellow' }}>Cairo VM </Text>
            <ColorSchemeToggle />
          </Group>
        </Container>
      </AppShell.Header>

      <AppShell.Main>
        <FloatingImage src='/images/space.png' width='400px' top='40px' right='20px' />
        {/* <FloatingImage src='/images/book.png' width='400px' bottom='10px' left='10px' /> */}
        <FloatingImage src='/images/star.png' width='100px' top='400px' right='50px' />
        <FloatingImage src='/images/star.png' width='150px' top='15px' right='15px' />
        <Container className="h-100" size={'xl'}>
          {children}
        </Container>
      </AppShell.Main>
    </AppShell>
  );
};

export default MainLayout;
