import * as vscode from 'vscode';
import { exec } from 'child_process';

export function activate(context: vscode.ExtensionContext) {

	let disposable = vscode.commands.registerCommand('cargo-cpack-vscode.cargoCpackToClipboard', () => {
		// アクティブなエディタからファイルのパスを取得
		const editor = vscode.window.activeTextEditor;
		if (!editor) {
			vscode.window.showErrorMessage('No active editor!');
			return;
		}

		const filePath = editor.document.uri.fsPath;

		// cargo-cpack コマンドを実行する
		exec(`cargo cpack -f ${filePath}`, (error, stdout, stderr) => {
			if (error) {
				vscode.window.showErrorMessage(`Error: ${error.message}`);
				return;
			}
			if (stderr) {
				vscode.window.showErrorMessage(`Stderr: ${stderr}`);
				return;
			}

			// 実行結果をクリップボードにコピー
			vscode.env.clipboard.writeText(stdout).then(() => {
				vscode.window.showInformationMessage('Result copied to clipboard!');
			});
		});
	});

	context.subscriptions.push(disposable);
}

// This method is called when your extension is deactivated
export function deactivate() { }
