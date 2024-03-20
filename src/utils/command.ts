import { defineInvoke } from 'manatsu';
import { Command as ManatsuCommand } from '@manatsu/tauri-plugin';

enum Command {
  Version = 'version'
}

export const useInvoke = defineInvoke({
  ...ManatsuCommand,
  ...Command
});
