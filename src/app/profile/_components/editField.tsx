import { Stack, styled } from '@/styling/jsx';
import { Input, TextArea } from '@/recipes/input';

export function EditField({
  label,
  value,
  fieldId,
  type,
  required,
}: {
  label: string;
  value?: string;
  fieldId: string;
  type?: string;
  required?: boolean;
}) {
  return (
    <Stack gap={0}>
      <styled.label htmlFor={label} mb={1} color={'text'}>
        {label} {required && '*'}
      </styled.label>
      {type === 'textarea' ? (
        <TextArea
          name={fieldId}
          id={label}
          defaultValue={value}
          required={required}
          rows={5}
          cols={30}
        />
      ) : (
        <Input
          type={type ?? 'text'}
          name={fieldId}
          id={label}
          defaultValue={value}
          required={required}
        />
      )}
    </Stack>
  );
}
