import NextAuth, { NextAuthConfig } from 'next-auth';

export const authOptions = {
  secret: process.env.NEXTAUTH_SECRET,
  session: { strategy: 'jwt' },
  callbacks: {
    session: async ({ session, token }) => {
      if (session?.user) {
        if (token.sub != null) {
          session.user.id = token.sub;
        }
      }
      return session;
    },
  },
  providers: [],
} satisfies NextAuthConfig;

export const { handlers, auth, signIn, signOut, update } =
  NextAuth(authOptions);
