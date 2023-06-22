import { Ref, ref } from "vue";

export type TypeNotification = "alert" | "error" | "info"; // "success"

export interface INotification {
  type: TypeNotification;
  content: string;
  title: string;
  debug?: string;
}

export interface INotifyId extends INotification {
  id: number;
}

export class NotificationState {
  private queue: Ref<INotifyId[]>;

  constructor() {
    this.queue = ref([]);
  }

  public notify(notification: INotification) {
    this.queue.value.push({
      ...notification,
      id: this.queue.value.length,
    });
  }

  public remove(notifyId: number) {
    const index = this.queue.value.findIndex(({ id }) => notifyId === id);
    if (index >= 0) {
      this.queue.value.splice(index, 1);
    }
  }

  public get notifications(): INotifyId[] {
    return this.queue.value;
  }
}

export const NotifyState = new NotificationState();
