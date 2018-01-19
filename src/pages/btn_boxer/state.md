# Сопровождение внешнего состояния

В этой главе у нас будет некоторое состояние, которым мы будем управлять с помощью UI. Поэтому нам необходим способ хранения и загрузки значения из этого состояния. Программа, которую мы хотим написать, имеет один компонент: значение здоровья.

Как оказалось, мы напрямую можем воспользоваться атомарными примитивами, таким как **AtomicUsize**, чтобы хранить значение для совместного использования нескольких неизменяемых замыканий. Этим атомарным значением можно управлять, не требуя изменяемого доступа к внутреннему значению. Таким образом, мы можем передавать неизменяемые ссылки на это значение и изменять его даже когда оно уже одолжено в нескольких местах одновременно.

```rust
pub struct HealthComponent(AtomicUsize);
```

Пока мы здесь, можем продолжить и написать некоторую логику для этой структуры в блоке реализации, используя следующие методы для здоровья:`initializing`,`subtracting` и `healing`.

```rust
impl HealthComponent {
    fn new(initial: usize) -> HealthComponent { HealthComponent(AtomicUsize::new(initial)) }

    fn get_health(&self) -> usize { self.0.load(Ordering::SeqCst) }

    fn subtract(&self, value: usize) -> usize {
        let current = self.0.load(Ordering::SeqCst);
        let new = if current < value { 0 } else { current - value };
        self.0.store(new, Ordering::SeqCst);
        new
    }

    fn heal(&self, value: usize) -> usize {
        let original = self.0.fetch_add(value, Ordering::SeqCst);
        original + value
    }
}
```
