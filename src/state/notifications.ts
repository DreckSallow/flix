import { Ref, ref } from "vue";

export type TypeNotification = "alert" | "error" | "info"; // "success"

export interface INotification {
  type: TypeNotification;
  content: string;
  title: string;
}

export class NotificationState {
  private queue: Ref<INotification[]>;

  constructor() {
    this.queue = ref([]);
  }

  public notify(notification: INotification) {
    console.log("NOTIFY METHOD!");
    this.queue.value.push(notification);
  }

  public remove(index: number) {
    this.queue.value = this.queue.value
      .slice(0, index)
      .concat(this.queue.value.slice(index + 1));
  }

  public get notifications(): INotification[] {
    return this.queue.value;
  }
}

export const NotifyState = new NotificationState();
