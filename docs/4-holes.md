1. pnpm 使用本地依赖

   ```json
   "react-arborist": "file:/home/chin/Projects/react-arborist/packages/react-arborist",
   ```

2. 发现更新本地依赖后，网页上仍然报错
```shell
rmi -rf node_modules/.vite/; pnpm run dev
```

package.json 中需要改为 force

``` json
  "scripts": {
    "build": "tsc && vite build",
    "dev": "vite --force --host",
    "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
    "preview": "vite preview"
  },
```