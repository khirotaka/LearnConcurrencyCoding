use std::sync::{Arc, Mutex};

struct Resource<const NRES: usize, const NTH: usize> {
    available: [usize; NRES],         // 利用可能なリソース
    allocation: [[usize; NRES]; NTH], // スレッドiが確保中のリソース
    max: [[usize; NRES]; NTH],        // スレッドiが必要とするリソースの最大値
}
// available[j] はj番目のリソース
// allocation[i][j] はスレッドiが現在確保しているリソースj の数を表現している。
// max[i][j] はスレッドiが必要とするリソースjの最大値

impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
    fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Resource {
            available,
            allocation: [[0; NRES]; NTH], // 0で初期化し、何も確保していないことを表現
            max,
        }
    }

    /// デットロック or 飢餓 に陥らない場合 true
    fn is_safe(&self) -> bool {
        let mut finish = [false; NTH];
        let mut work = self.available.clone();

        loop {
            // finish[i] == false && work[j] >= (self.max[i][j] - self.allocation[i][j])
            // を満たすようなスレッドを見つける。
            let mut found = false;
            let mut num_true = 0;

            for (i, alc) in self.allocation.iter().enumerate() {
                if finish[i] {
                    num_true += 1;
                    continue;
                }
                // (m ... maxの要素, a ... alcの要素)
                let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
                // need[j] = self.max[i][j] - self.allocation[i][j] を計算
                let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
                // ここまでで、work[j] >= (self.max[i][j] - self,allocation[i][j])
                // all() イテレータの全ての要素が条件を満たすかどうか
                if is_avail {
                    found = true;
                    finish[i] = true;
                    for (w, a) in work.iter_mut().zip(alc) {
                        *w += *a;   // 現在利用可能なリソース量を更新(返却する)
                    }
                    break;
                }
            }

            if num_true == NTH {    // 全てのスレッドがリソース確保可能なら安全
                return true;
            }

            if !found {
                break;
            }
        }

        false
    }
    /// id番目のスレッドが、resourceを1つ取得
    fn take(&mut self, id: usize, resource: usize) -> bool {
        if id >= NTH
            || resource >= NRES
            || self.available[resource] == 0
            || self.max[id][resource] == self.allocation[id][resource]
        {
            return false;
        }

        self.allocation[id][resource] += 1;
        self.available[resource] -= 1;

        if self.is_safe() {
            true
        } else {
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
            false
        }
    }

    fn release(&mut self, id: usize, resource: usize) {
        if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
            return;
        }
        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
    }
}

#[derive(Clone)]
pub struct Banker<const NRES: usize, const NTH: usize> {
    resource: Arc<Mutex<Resource<NRES, NTH>>>,
}

impl<const NRES: usize, const NTH: usize> Banker<NRES, NTH> {
    pub fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Banker {
            resource: Arc::new(Mutex::new(Resource::new(available, max))),
        }
    }

    pub fn take(&self, id: usize, resource: usize) -> bool {
        let mut r = self.resource.lock().unwrap();
        r.take(id, resource)
    }

    pub fn release(&self, id: usize, resource: usize) {
        let mut r = self.resource.lock().unwrap();
        r.release(id, resource);
    }
}
