import TokenForm from '@/components/Tokens/TokenForm';
import { Welcome } from '../components/Welcome/Welcome';

export function HomePage() {
  return (
    <>
      <Welcome />
      <TokenForm />
    </>
  );
}
