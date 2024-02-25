import { ReactNode } from 'react';
import { styled } from '@/styling/jsx';

export async function AdminTable(props: {
  dataList: any[];
  callbackFn: (item: any) => ReactNode;
  header: string[];
}) {
  return (
    <styled.table
      w={'full'}
      css={{
        '& th,td': {
          textAlign: 'left',
          p: 1,
          outline: 'none',
        },
        '& th:not(:last-child),td:not(:last-child)': {
          borderRight: '1px solid lightgray',
        },
        '& thead': {
          _first: {
            borderBottom: '1px solid lightgray',
          },
        },
        '& tbody tr': {
          _odd: {
            bgct: 'primary/90',
          },
          cursor: 'pointer',
          _hover: {
            bgct: 'secondary/90',
          },
        },
      }}>
      <styled.thead>
        <tr>
          {props.header.map((item: string) => (
            <th key={`table-header-${item}`}>{item}</th>
          ))}
        </tr>
      </styled.thead>
      <tbody>{props.dataList.map(props.callbackFn)}</tbody>
    </styled.table>
  );
}
