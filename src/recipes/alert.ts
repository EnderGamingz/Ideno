import { cva } from '@/styling/css';
import { styled } from '@/styling/jsx';

export const alert = cva({
  base: {
    display: 'flex',
    alignItems: 'center',
    borderRadius: 'sm',
    py: 2,
    px: 3,
    gap: 3,
    transition: 'all 0.2s ease-in-out',
    overflow: 'hidden',
  },
  variants: {
    status: {
      info: {
        outline: '1px solid',
        oct: 'secondary/40',
        bgt: 'secondary/70',
        color: 'text.secondary',
      },
      success: {
        outline: '1px solid',
        oct: 'success/40',
        bgt: 'success/70',
        color: 'text.success',
      },
      warning: {
        outline: '1px solid',
        oct: 'warning/40',
        bgt: 'warning/70',
        color: 'text.warning',
      },
      error: {
        outline: '1px solid',
        oct: 'error/40',
        bgt: 'error/70',
        color: 'text.error',
      },
      hidden: {
        height: 0,
        opacity: 0,
        visibility: 'hidden',
        pointerEvents: 'none',
        userSelect: 'none',
      },
    },
  },

  defaultVariants: {
    status: 'info',
  },
});

const Alert = styled('div', alert);
export default Alert;
