import { cva } from '@/styling/css';
import { styled } from '@/styling/jsx';

export const chip = cva({
  base: {
    display: 'flex',
    gap: 1,
    alignItems: 'center',
    justifyContent: 'center',
    p: '0 12px',
    fontSize: '16px',
    borderRadius: '16px',
    color: 'white',
    bg: '#e0e0e0',
    boxShadow: 'none',
    h: '32px',
    transition: 'all 0.2s ease-in-out',
    '&:hover': {
      bg: '#d1d1d1',
      boxShadow: '0 2px 10px 0 rgba(0, 0, 0, 0.1)',
    },
    '&:focus': {
      boxShadow: '0 2px 10px 0 rgba(0, 0, 0, 0.1)',
    },
  },
});

const Chip = styled('button', chip);
export default Chip;
