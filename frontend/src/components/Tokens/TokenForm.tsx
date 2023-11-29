import { CodeHighlight } from '@mantine/code-highlight';
import { Button, Textarea, Grid } from '@mantine/core';
import { useForm } from '@mantine/form';
import axios from 'axios';
import React, { useState } from 'react';

const TokenForm = () => {
  const [result, setResult] = useState<null | any>('');
  const form = useForm({
    initialValues: {
      code: '',
    },

    validate: {
      code: (value) => (value === '' ? 'code is required' : null),
    },
  });

  const handleSubmit = () => {
    console.log(form.values);
    let data = JSON.stringify({
      code: form.values.code,
    });

    let config = {
      method: 'post',
      maxBodyLength: Infinity,
      url: 'http://127.0.0.1:8080/tokenize',
      headers: {
        'Content-Type': 'application/json',
      },
      data: data,
    };

    axios
      .request(config)
      .then((response) => {
        console.log(JSON.stringify(response.data));
        setResult(response)
      })
      .catch((error) => {
        console.log(error);
      });
  };

  return (
    <div>
      <form onSubmit={form.onSubmit((values) => handleSubmit())}>
        <Textarea
          label="write Cairo code"
          autosize
          minRows={5}
          radius={'md'}
          {...form.getInputProps('code')}
        />
        <Button mt="md" radius={'md'} type="submit">
          Tokenize
        </Button>
      </form>

      <Grid>
        <Grid.Col span={{ md: 6 }}>
          <CodeHighlight
            code={form.values.code}
            language="rust"
            copyLabel="Copy button code"
            copiedLabel="Copied!"
          />
        </Grid.Col>

        <Grid.Col span={{ md: 6 }}>
          {result ? (
            <CodeHighlight
              code={JSON.stringify(result, null, 4)}
              language="json"
              copyLabel="Copy button code"
              copiedLabel="Copied!"
            />
          ) : null}
        </Grid.Col>
      </Grid>
    </div>
  );
};

export default TokenForm;
