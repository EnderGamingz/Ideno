import Icon from '@/app/_components/icon';
import LogoutAction from '@/app/_components/header/logoutAction';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';

export default function LogoutButton() {
  return (
    <ConfirmPopover
      label={'Are you sure you want to logout?'}
      buttonEl={<Icon.Logout />}
      confirm={{
        action: LogoutAction,
        button: (
          <>
            <Icon.Logout /> Logout
          </>
        ),
      }}
    />
  );
}
