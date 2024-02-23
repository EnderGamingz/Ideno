import { AuthContactInformationModel } from '@/types/contactInformation';
import { HStack, Stack, styled } from '@/styling/jsx';
import Icon from '@/app/_components/icon';
import { OutsideLinkConditionalWrapper } from '@/app/_components/ConditionalWrapper';
import EditContactInformationDialog from '@/app/profile/_components/contactInformation/editContactInformationDialog';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';
import deleteContactInformationAction from '@/app/profile/_components/contactInformation/deleteContactInformationAction';

export const contactTypes: ((string | any)[] | string[])[] = [
  [
    'email',
    'Email',
    {
      type: 'email',
      prefix: 'mailto:',
      clickable: true,
    },
  ],
  [
    'phone',
    'Phone',
    {
      type: 'tel',
      prefix: 'tel:',
      clickable: true,
    },
  ],
  [
    'website',
    'Website',
    {
      type: 'text',
      prefix: 'https://',
      visiblePrefix: 'https://',
      clickable: true,
    },
  ],
  ['linkedin', 'LinkedIn'],
  ['github', 'GitHub'],
  ['twitter', 'Twitter'],
  ['facebook', 'Facebook'],
  ['instagram', 'Instagram'],
];

function getContactIcon(type: string | undefined) {
  switch (type) {
    case 'email':
      return Icon.Mail;
    case 'phone':
      return Icon.Phone;
    case 'website':
      return Icon.Web;
    case 'linkedin':
      return Icon.Linkedin;
    case 'github':
      return Icon.Github;
    case 'twitter':
      return Icon.Twitter;
    case 'facebook':
      return Icon.Facebook;
    case 'instagram':
      return Icon.Instagram;
    default:
      return Icon.Error;
  }
}

export function ContactInformationItem({
  item,
  full,
  last,
}: {
  item: AuthContactInformationModel;
  full?: boolean;
  last?: boolean;
}) {
  const type = contactTypes.find(x => x[0] === item.type_field);
  const contactIcon = getContactIcon(item.type_field)({ size: 22 });
  let isClickable = type?.[2]?.clickable;
  let contactLink = type?.[2]?.prefix + item.value;
  return (
    <OutsideLinkConditionalWrapper
      condition={isClickable && !full}
      href={contactLink}>
      <Stack
        p={2}
        transition={'all 0.2s'}
        css={{
          borderBottom: full && !last ? '1px solid' : 'none',
          border: !full ? '1px solid' : 'none',
          rounded: !full ? 'md' : 'none',
          bct: 'black/90',
        }}
        _hover={
          isClickable &&
          !full && {
            shadow: 'md',
            bgct: 'primary/90',
          }
        }>
        <HStack gap={0}>
          {contactIcon}
          <styled.h3 ml={2} mr={'auto'} fontSize={'1.3rem'}>
            {type?.[1] ?? item.type_field}
          </styled.h3>
          <HStack>
            {isClickable && (
              <OutsideLinkConditionalWrapper
                condition={full}
                href={contactLink}>
                <Icon.OpenInNew size={full ? 22 : 15} />
              </OutsideLinkConditionalWrapper>
            )}
            {item.id && (
              <>
                <EditContactInformationDialog data={item} />{' '}
                <ConfirmPopover
                  label={'Are you sure?'}
                  buttonEl={<Icon.Delete />}
                  confirm={{
                    action: deleteContactInformationAction,
                    actionPayload: item.id,
                    refresh: true,
                    button: (
                      <>
                        <Icon.Delete /> Delete
                      </>
                    ),
                  }}
                />
              </>
            )}
          </HStack>
        </HStack>
        <Stack gap={0} px={1}>
          <styled.span fontWeight={'bold'}>
            {type?.[2]?.visiblePrefix}
            {item.value}
          </styled.span>
        </Stack>
      </Stack>
    </OutsideLinkConditionalWrapper>
  );
}
