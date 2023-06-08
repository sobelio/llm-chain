import React from 'react';
import EnhancedChat from 'enhancedocs-chat';

import 'enhancedocs-chat/dist/style.css';

// Default implementation, that you can customize
export default function Root({ children }) {
  return (
    <>
      {children}
      <EnhancedChat
        config={{
          projectId: "64456d414958ccf11d241c33",
          accessToken: "pk_8dcb26507945f1d762962fb6bbe1ca9a058b6af3f07b80b5"
        }}
        size="large"
      />
    </>
  );
}
