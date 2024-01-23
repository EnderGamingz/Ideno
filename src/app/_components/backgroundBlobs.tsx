import { styled } from '@/styling/jsx';

export function BackgroundBlobs() {
  return (
    <styled.svg
      css={{
        position: 'fixed',
        inset: 0,
        pointerEvents: 'none',
        width: '100%',
        height: '100%',
        zIndex: -1,
      }}
      xmlns='http://www.w3.org/2000/svg'
      version='1.1'
      viewBox='0 0 800 450'
      preserveAspectRatio='none'>
      <defs>
        <filter
          id='blurry-filter'
          x='-100%'
          y='-100%'
          width='400%'
          height='400%'
          filterUnits='objectBoundingBox'
          primitiveUnits='userSpaceOnUse'>
          <feGaussianBlur
            stdDeviation='118'
            x='0%'
            y='0%'
            width='100%'
            height='100%'
            in='SourceGraphic'
            edgeMode='none'
            result='blur'
          />
        </filter>
      </defs>
      <g filter='url(#blurry-filter)'>
        <ellipse
          rx='83'
          ry='84'
          cx='636.2961592240766'
          cy='-38.71934370561081'
          fill='#60c7c7'></ellipse>
        <ellipse
          rx='83'
          ry='84'
          cx='679.0777365944602'
          cy='357.43588811701'
          fill='#7e9dec'></ellipse>
        <ellipse
          rx='83'
          ry='84'
          cx='143.90559942072088'
          cy='240.83058305220172'
          fill='hsla(254, 43%, 49%, 1.00)'></ellipse>
      </g>
    </styled.svg>
  );
}
