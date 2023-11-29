import { ActionIcon, Button, Group, useMantineColorScheme } from '@mantine/core';
import { IconSun, IconMoon } from "@tabler/icons-react"

export function ColorSchemeToggle() {
  const { colorScheme, setColorScheme } = useMantineColorScheme();

  return (
    <Group justify="center" >
      <ActionIcon size={'lg'} variant='subtle' radius={'md'} onClick={() => setColorScheme(colorScheme === 'dark' ? 'light' : 'dark')}>
        {colorScheme === 'dark' ? <IconSun color='orange' /> : <IconMoon color='black' />}
      </ActionIcon>
    </Group>
  );
}
