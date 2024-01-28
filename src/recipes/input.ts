import { cva } from '@/styling/css';
import { styled } from '@/styling/jsx';

export const input = cva({
  base: {
    outline: '1px solid',
    oct: 'primary/85',
    rounded: 'md',
    shadow: 'md',
    px: 2,
    py: 1,
    resize: 'none',
  },
});

const Input = styled('input', input);
const TextArea = styled('textarea', input);
const Select = styled('select', input);
export { Input, TextArea, Select };
