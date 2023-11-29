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
        <FloatingImage src='/images/img.png' width='100px' top='50px' right='50px' />
        <FloatingImage src='/images/img.png' width='200px' bottom='20px' left='20px' />
        {/* <FloatingImage src='/images/img.png' width='200px' top='400px' left='50px' />
        <FloatingImage src='/images/img.png' width='100px' top='300px' left='300px' /> */}
        <Container className="h-100" size={'xl'}>
          {children}
        </Container>
      </AppShell.Main>
    </AppShell>
  );
};

export default MainLayout;
