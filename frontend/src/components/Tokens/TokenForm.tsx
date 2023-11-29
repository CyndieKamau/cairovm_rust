import { CodeHighlight } from '@mantine/code-highlight';
import { Button, Textarea, Grid, Card, useMantineColorScheme, LoadingOverlay, Title, Spoiler, Box, Stack, Image } from '@mantine/core';
import { useForm } from '@mantine/form';
import axios from 'axios';
import React, { useState } from 'react';

const TokenForm = () => {
  const [result, setResult] = useState<null | any>('');
  const [loading, setLoading] = useState<boolean>(false)
  const { colorScheme } = useMantineColorScheme()
  const form = useForm({
    initialValues: {
      code: '',
    },

    validate: {
      code: (value) => (value === '' ? 'code is required' : null),
    },
  });

  const handleSubmit = () => {
    let data = JSON.stringify({
      code: form.values.code,
    });

    let config = {
      method: 'post',
      maxBodyLength: Infinity,
      url: 'https://cairovm.supercodehive.com/tokenize',
      headers: {
        'Content-Type': 'application/json',
      },
      data: data,
    };
    setLoading(true)
    axios
      .request(config)
      .then((response) => {
        setResult(response.data)
      })
      .catch((error) => {
        console.log(error);
      }).finally(() => {
        setLoading(false)
      })
  };

  return (
    <div>
      <Card radius={'md'} style={theme => ({
        background: colorScheme === 'dark' ? theme.colors.dark[5] : theme.colors.gray[2],
        position: "relative"
      })}>
        <LoadingOverlay visible={loading} />
        <form onSubmit={form.onSubmit((values) => handleSubmit())}>
          <Textarea
            placeholder='Write Cairo Code here'
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
      </Card>
      {result ? (
        <Grid mt={'lg'} mb={400}>
          <Grid.Col span={{ md: 12 }}>
            <Stack gap={10}>
              <Title order={2} fw={500} mb={'lg'}>Code</Title>
              <Spoiler maxHeight={320} showLabel="Show more" hideLabel="Hide">
                <CodeHighlight
                  code={form.values.code}
                  language="rust"
                  copyLabel="Copy button code"
                  copiedLabel="Copied!"
                />
              </Spoiler>
              <Title order={2} fw={500} mb={'lg'}>Tokens</Title>
              <Spoiler maxHeight={320} showLabel="Show more" hideLabel="Hide">
                <CodeHighlight
                  code={JSON.stringify(result, null, 4)}
                  language="json"
                  copyLabel="Copy button code"
                  copiedLabel="Copied!"
                />
              </Spoiler>
            </Stack>
          </Grid.Col>
        </Grid >
      ) : null}
      <Box style={{ height: " 400px" }}></Box>
    </div >
  );
};

export default TokenForm;
