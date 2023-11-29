import '@mantine/core/styles.css';
import '@mantine/notifications/styles.css';
import '@mantine/code-highlight/styles.css';
//import '@mantine/modals/styles.css';
import './styles/main.css';
import { MantineProvider } from '@mantine/core';
import { Router } from './Router';
import { theme } from './theme';
import MainLayout from './layout/MainLayout';
import { ModalsProvider } from '@mantine/modals';
import { Notifications } from '@mantine/notifications';


export default function App() {
  return (
    <MantineProvider theme={theme}>
      <Notifications />
      <ModalsProvider>
        
        <MainLayout>
          <Router />

        </MainLayout>     

      </ModalsProvider>
       
    </MantineProvider>
  );
}
