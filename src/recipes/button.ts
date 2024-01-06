import { cva } from '@/styling/css';
import { styled } from '@/styling/jsx';

export const button = cva({
  base: {
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    rounded: 'sm',
    fontSize: '18px',
    py: 1,
    gap: 2,
    shadow: 'md',
    cursor: 'pointer',
    fontWeight: '500',
    transition: 'all 0.2s ease-in-out',
    _hover: {
      shadow: 'xl',
    },
  },
});

const Button = styled('button', button);
export default Button;
