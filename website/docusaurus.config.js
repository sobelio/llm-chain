// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github')
const darkCodeTheme = require('prism-react-renderer/themes/dracula')

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'llm-chain',
  tagline: 'Unlock the full potential of Large Language Models',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://docs.llm-chain.xyz/',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'sobelio', // Usually your GitHub org/user name.
  projectName: 'llm-chain', // Usually your repo name.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en']
  },
  scripts: [{ src: 'https://embed.lu.ma/checkout-button.js', async: true, defer: true }],

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: 'https://github.com/sobelio/llm-chain/tree/main/docs/'
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: 'https://github.com/sobelio/llm-chain/tree/main/blog/'
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css')
        },
        gtag: {
          trackingID: 'G-CVN7CRKQ8Z',
          anonymizeIP: true
        }
      })
    ]
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: 'img/llmchainsocial.png',
      navbar: {
        logo: {
          alt: 'llm-chain-logo',
          src: 'img/llmchain.png'
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'sidebar',
            position: 'left',
            label: 'Documentation'
          },
          { to: '/blog', label: 'Blog', position: 'left' },
          {
            href: 'https://github.com/sobelio/llm-chain',
            label: 'GitHub',
            position: 'right'
          }
        ]
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {
                label: 'Introduction',
                to: '/docs/introduction'
              }
            ]
          },
          {
            title: 'Community',
            items: [
              {
                label: 'Discord',
                href: 'https://discord.gg/kewN9Gtjt2'
              },
              {
                label: 'GitHub',
                href: 'https://github.com/sobelio/llm-chain'
              },
              {
                label: 'Events',
                href: '/llmcasual'
              }
            ]
          },
          {
            title: 'More',
            items: [
              {
                label: 'Blog',
                to: '/blog'
              },
              {
                label: 'Docs.rs',
                href: 'https://docs.rs/llm-chain'
              },
              {
                label: 'Crates.io',
                href: 'https://crates.io/crates/llm-chain'
              },
              {
                label: 'Sobel.io',
                href: 'https://sobel.io'
              }
            ]
          }
        ],
        copyright: `Copyright © ${new Date().getFullYear()} - sobel.io`
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ['rust']
      }
    })
}

module.exports = config
