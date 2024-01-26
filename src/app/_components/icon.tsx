const Icon = {
  Logout: ({ size = 20 }: { size?: number }) => (
    <svg
      xmlns='http://www.w3.org/2000/svg'
      height={size}
      width={size}
      viewBox={`0 0 24 24`}
      fill='currentColor'>
      <path d='M0 0h24v24H0z' fill='none' />
      <path d='M17 7l-1.41 1.41L18.17 11H8v2h10.17l-2.58 2.58L17 17l5-5zM4 5h8V3H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8v-2H4V5z' />
    </svg>
  ),
  Login: ({ size = 20 }: { size?: number }) => (
    <svg
      xmlns='http://www.w3.org/2000/svg'
      height={size}
      width={size}
      viewBox='0 0 24 24'
      fill='currentColor'>
      <g>
        <rect fill='none' height='24' width='24' />
      </g>
      <g>
        <path d='M11,7L9.6,8.4l2.6,2.6H2v2h10.2l-2.6,2.6L11,17l5-5L11,7z M20,19h-8v2h8c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2h-8v2h8V19z' />
      </g>
    </svg>
  ),
  Error: ({ size = 20 }: { size?: number }) => (
    <svg
      xmlns='http://www.w3.org/2000/svg'
      height={size}
      width={size}
      viewBox='0 0 24 24'
      fill='currentColor'>
      <path d='M0 0h24v24H0z' fill='none' />
      <path d='M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z' />
    </svg>
  ),
  Check: ({ size = 20 }: { size?: number }) => (
    <svg
      xmlns='http://www.w3.org/2000/svg'
      height={size}
      width={size}
      viewBox='0 0 24 24'
      fill='currentColor'>
      <path d='M0 0h24v24H0z' fill='none' />
      <path d='M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z' />
    </svg>
  ),
  Back: ({ size = 20 }: { size?: number }) => (
    <svg
      xmlns='http://www.w3.org/2000/svg'
      height={size}
      width={size}
      viewBox='0 0 24 24'
      fill='currentColor'>
      <path d='M0 0h24v24H0z' fill='none' />
      <path d='M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z' />
    </svg>
  ),
  Edit: ({ size = 20 }: { size?: number }) => (
    <svg
      xmlns='http://www.w3.org/2000/svg'
      height={size}
      width={size}
      viewBox='0 -960 960 960'
      fill='currentColor'>
      <path d='M200-200h57l391-391-57-57-391 391v57Zm-80 80v-170l528-527q12-11 26.5-17t30.5-6q16 0 31 6t26 18l55 56q12 11 17.5 26t5.5 30q0 16-5.5 30.5T817-647L290-120H120Zm640-584-56-56 56 56Zm-141 85-28-29 57 57-29-28Z' />
    </svg>
  ),
};

export default Icon;
