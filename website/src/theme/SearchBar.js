import React from 'react';
import EnhancedSearch from 'enhancedocs-search';

import 'enhancedocs-search/dist/style.css';

export default function SearchBarWrapper(props) {
  return (
    <EnhancedSearch
      config={{
        enhancedSearch: {
          projectId: "64456d414958ccf11d241c33",
          accessToken: "pk_8dcb26507945f1d762962fb6bbe1ca9a058b6af3f07b80b5"
        }
      }}
      {...props}
    />
  );
}
