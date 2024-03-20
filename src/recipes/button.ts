import { cva } from '@/styling/css';
import { styled } from '@/styling/jsx';

export const button = cva({
  base: {
    display: 'flex',
    alignItems: 'center',
    borderRadius: 'full',
    py: '8px',
    px: '18px',
    gap: 2,
    cursor: 'pointer',
    fontWeight: 'semibold',
    transition: 'all 0.2s ease-in-out',
    fontSize: '16px',
    _hover: {
      boxShadow: '0 2px 10px 0 rgba(0,0,0,0.2)',
      outline: '1px solid black',
      outlineOffset: '-3px',
    },
  },
  variants: {
    variant: {
      solid: {
        bg: 'primary',
        color: 'white',
        _hover: {
          bg: 'primary.hover',
          color: 'text.hover',
        },
      },
      outline: {
        outline: '2px solid token(colors.primary)',
        outlineOffset: '-1px',
        bg: 'transparent',
        color: 'primary',
        _hover: {
          bg: 'primary.hover',
          color: 'text.hover',
        },
      },
      secondary: {
        bg: 'transparent',
        color: 'text',
        outline: '1px solid',
        oct: 'black/80',
        _hover: {
          bg: 'secondary.hover',
          color: 'text.hover',
        },
      },
    },
    contentType: {
      icon: {
        px: '8px',
        py: '8px',
      },
    },
    pending: {
      true: {
        bg: 'primary',
        color: 'text.primary',
        _hover: {
          bg: 'primary',
          color: 'text.primary',
        },
        cursor: 'wait',
        filter: 'grayscale(80%)',
      },
    },
    disabled: {
      true: {
        opacity: 0.5,
        cursor: 'not-allowed',
      },
    },
    size: {
      small: {
        px: 2,
        py: 1,
        fontSize: '0.8rem',
      },
      large: {
        py: '10px',
        px: '22px',
        fontSize: '1.2rem',
      },
    },
  },
  defaultVariants: {
    variant: 'solid',
  },
});

const Button = styled('button', button);
export default Button;
