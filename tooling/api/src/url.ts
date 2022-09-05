import { invokeTauriCommand } from './helpers/tauri'

/**
 * This function ensures that path is resolved absolutely before encodeding and converting into a File URL.
 *
 * @example
 * ```typescript
 * import { pathToFileURL } from '@tauri-apps/api/url';
 * const url = await pathToFileURL("/foo#1");
 * ```
 */
async function pathToFileURL(path: string): Promise<URL> {
  const url = await invokeTauriCommand<string>({
    __tauriModule: 'Url',
    message: {
      cmd: 'pathToFileURL',
      path
    }
  })
  return new URL(url, 'file:')
}

export { pathToFileURL }
