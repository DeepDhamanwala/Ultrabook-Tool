import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { invoke } from '@tauri-apps/api/core'; // Note: check your tauri version for import path
import { FormsModule } from '@angular/forms';

interface Diagnostics {
  cpu_temp: number;
  battery_health: number;
  fan_rpm: number;
  model: string;
}

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, FormsModule],
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  diagnostics: Diagnostics | null = null;
  statusMessage = 'System Ready';
  fanProfile = 'Silent';
  backlightLevel = 50;

  async loadDiagnostics() {
    this.statusMessage = 'Scanning Hardware...';
    try {
      this.diagnostics = await invoke<Diagnostics>('get_diagnostics');
      this.statusMessage = `Connected to: ${this.diagnostics.model}`;
    } catch (error) {
      this.statusMessage = 'Error scanning hardware';
    }
  }

  async setFanProfile(profile: string) {
    this.fanProfile = profile;
    this.statusMessage = await invoke('set_fan_curve', { profile });
  }

  async updateBacklight() {
    this.statusMessage = await invoke('set_backlight', { brightness: this.backlightLevel });
  }

  async remapSearchKey() {
    this.statusMessage = await invoke('remap_key', { source: 'Search', target: 'CapsLock' });
  }
}