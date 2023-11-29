import { AppShell, Group, Title, Container } from '@mantine/core';
import React, { Children } from 'react';
import { ColorSchemeToggle } from '../components/ColorSchemeToggle/ColorSchemeToggle';

interface ImainLayout {
  children: React.ReactNode;
}
const MainLayout = (props: ImainLayout) => {
  const { children } = props;
  return (
    <AppShell header={{ height: { base: '70px' } }}>
      <AppShell.Header>
        <Container className="h-100" size={'xl'}>
          <Group className="h-100" align="center" justify='space-between'>
          
            <Title>Cairo VM </Title>
            <ColorSchemeToggle/>
          </Group>
        </Container>
      </AppShell.Header>

      <AppShell.Main>
        <Container className="h-100" size={'xl'}>
          {children}
        </Container>
      </AppShell.Main>
    </AppShell>
  );
};

export default MainLayout;
